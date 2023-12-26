use crate::{PowerSupplyError, PowerSupplyErrorCodes};
use anyhow::{bail, Result};
use std::fs::{self, File};
use std::io::Read;
use tracing::{error as trace_error, info, instrument, trace};

#[derive(Debug)]
pub struct BatteryControl {
    pub name: String,
    pub r#type: String,
    pub status: String,
    pub present: bool,
    pub voltage_now: u32,
    pub current_now: i32,
    pub capacity: u8,
    pub capacity_level: String,
    pub temp: i32,
    pub technology: String,
    pub charge_full: u32,
    pub charge_now: u32,
    pub charge_full_design: u32,
    pub manufacturer: String,
}

pub trait PowerSupplyInfo {
    fn info(&self) -> Result<BatteryControl>;
    fn set_device(&mut self, device: &str) -> Result<()>;
    fn get_device(&self) -> Result<&str>;
    fn get_current(&self) -> Result<i64>;
}

#[derive(Debug, Default)]
pub struct Battery {
    pub path: String,
    pub currnet_now: String,
}

impl PowerSupplyInfo for Battery {
    #[instrument]
    fn info(&self) -> Result<BatteryControl> {
        trace!(task = "battery_info", "init");
        info!("Battery info");
        let mut file = match File::open(&self.path) {
            Ok(file) => {
                trace!(task = "battery_info", "open file");
                file
            }
            Err(err) => {
                trace_error!(task = "battery_info", "failed to open file: {}", err);
                bail!(PowerSupplyError::new(
                    PowerSupplyErrorCodes::FailedToOpenFile,
                    "failed to open file".to_string(),
                ))
            }
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut power_supply = BatteryControl {
            name: String::new(),
            r#type: String::new(),
            status: String::new(),
            present: false,
            voltage_now: 0,
            current_now: 0,
            capacity: 0,
            capacity_level: String::new(),
            temp: 0,
            technology: String::new(),
            charge_full: 0,
            charge_now: 0,
            charge_full_design: 0,
            manufacturer: String::new(),
        };

        for line in contents.lines() {
            let mut parts = line.splitn(2, '=');
            let key = parts.next().unwrap_or("").trim();
            let value = parts.next().unwrap_or("").trim();

            match key {
                "POWER_SUPPLY_NAME" => power_supply.name = value.to_string(),
                "POWER_SUPPLY_TYPE" => power_supply.r#type = value.to_string(),
                "POWER_SUPPLY_STATUS" => power_supply.status = value.to_string(),
                "POWER_SUPPLY_PRESENT" => power_supply.present = value == "1",
                "POWER_SUPPLY_VOLTAGE_NOW" => power_supply.voltage_now = value.parse().unwrap_or(0),
                "POWER_SUPPLY_CURRENT_NOW" => power_supply.current_now = value.parse().unwrap_or(0),
                "POWER_SUPPLY_CAPACITY" => power_supply.capacity = value.parse().unwrap_or(0),
                "POWER_SUPPLY_CAPACITY_LEVEL" => power_supply.capacity_level = value.to_string(),
                "POWER_SUPPLY_TEMP" => power_supply.temp = value.parse().unwrap_or(0),
                "POWER_SUPPLY_TECHNOLOGY" => power_supply.technology = value.to_string(),
                "POWER_SUPPLY_CHARGE_FULL" => power_supply.charge_full = value.parse().unwrap_or(0),
                "POWER_SUPPLY_CHARGE_NOW" => power_supply.charge_now = value.parse().unwrap_or(0),
                "POWER_SUPPLY_CHARGE_FULL_DESIGN" => {
                    power_supply.charge_full_design = value.parse().unwrap_or(0)
                }
                "POWER_SUPPLY_MANUFACTURER" => power_supply.manufacturer = value.to_string(),
                _ => {}
            }
        }

        Ok(power_supply)
    }

    #[instrument]
    fn set_device(&mut self, device: &str) -> Result<()> {
        //try to set path or return error
        trace!(task = "set_device", "init");
        info!(task = "set_devide", "set power device path");
        if device.is_empty() {
            trace_error!("Device path is empty");
            bail!(PowerSupplyError::new(
                PowerSupplyErrorCodes::FailedToOpenFile,
                "failed to set device".to_string(),
            ));
        }
        self.path = device.to_owned();
        Ok(())
    }

    #[instrument]
    fn get_device(&self) -> Result<&str> {
        if self.path.is_empty() {
            trace_error!(task = "get_device_path", "Device path is empty");
            bail!(PowerSupplyError::new(
                PowerSupplyErrorCodes::FailedToOpenFile,
                "Device path is empty".to_string(),
            ));
        }
        Ok(&self.path)
    }

    //to get current_now value read file from current_now path
    #[instrument]
    fn get_current(&self) -> Result<i64> {
        let mut file = fs::File::open(&self.currnet_now)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let current_now = match contents.trim().parse::<i64>() {
            Ok(value) => value,
            Err(_) => {
                trace_error!(task = "get_current", "Failed to parse current_now value");
                bail!(PowerSupplyError::new(
                    PowerSupplyErrorCodes::InvalidDataFormat,
                    "Failed to parse current_now value".to_string()
                ))
            }
        };
        Ok(current_now)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_battery_info() {
        let mut battery = Battery::default();

        // Create temp file with test data for all fields
        let mut tmpfile = NamedTempFile::new().unwrap();
        writeln!(tmpfile, "POWER_SUPPLY_NAME=BAT0").unwrap();
        writeln!(tmpfile, "POWER_SUPPLY_VOLTAGE_NOW=12157000").unwrap();
        writeln!(tmpfile, "POWER_SUPPLY_TYPE=Battery").unwrap();
        writeln!(tmpfile, "POWER_SUPPLY_STATUS=Discharging").unwrap();
        writeln!(tmpfile, "POWER_SUPPLY_PRESENT=1").unwrap();
        writeln!(tmpfile, "POWER_SUPPLY_CURRENT_NOW=0").unwrap();
        writeln!(tmpfile, "POWER_SUPPLY_CAPACITY=100").unwrap();
        writeln!(tmpfile, "POWER_SUPPLY_CAPACITY_LEVEL=Normal").unwrap();
        writeln!(tmpfile, "POWER_SUPPLY_TEMP=30000").unwrap();
        writeln!(tmpfile, "POWER_SUPPLY_TECHNOLOGY=Li-ion").unwrap();
        writeln!(tmpfile, "POWER_SUPPLY_CHARGE_FULL=4400000").unwrap();
        writeln!(tmpfile, "POWER_SUPPLY_CHARGE_NOW=4400000").unwrap();
        writeln!(tmpfile, "POWER_SUPPLY_CHARGE_FULL_DESIGN=4400000").unwrap();
        writeln!(tmpfile, "POWER_SUPPLY_MANUFACTURER=SMP").unwrap();

        battery.path = tmpfile.path().to_str().unwrap().to_string();

        let power_supply = battery.info().unwrap();

        // Assert name parses correctly
        assert_eq!(power_supply.name, "BAT0");

        // Assert voltage parses correctly
        assert_eq!(power_supply.voltage_now, 12157000);
        // Assert additional fields...
        assert_eq!(power_supply.r#type, "Battery");
        assert_eq!(power_supply.status, "Discharging");
        assert_eq!(power_supply.present, true);
        assert_eq!(power_supply.current_now, 0);
        assert_eq!(power_supply.capacity, 100);
        assert_eq!(power_supply.capacity_level, "Normal");
        assert_eq!(power_supply.temp, 30000);
        assert_eq!(power_supply.technology, "Li-ion");
        assert_eq!(power_supply.charge_full, 4400000);
        assert_eq!(power_supply.charge_now, 4400000);
        assert_eq!(power_supply.charge_full_design, 4400000);
    }

    #[test]
    fn test_set_device() {
        let mut battery = Battery::default();
        battery.set_device("test_path").unwrap();

        assert_eq!(battery.path, "test_path");
    }

    #[test]
    fn test_get_device() {
        let mut battery = Battery::default();
        battery.path = "test_path".to_string();

        assert_eq!(battery.get_device().unwrap(), "test_path");
    }

    #[test]
    fn test_get_current() {
        let mut battery = Battery::default();

        // Create temp current file

        let mut tmpfile = NamedTempFile::new().unwrap();
        writeln!(tmpfile, "12345").unwrap();

        battery.currnet_now = tmpfile.path().to_str().unwrap().to_string();

        let current = battery.get_current().unwrap();

        // Assert
        assert_eq!(current, 12345);
    }
}
