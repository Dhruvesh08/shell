use anyhow::{bail, Result};
use clap::{Args, Subcommand};

pub use mecha_battery_ctl::{Battery as Power, PowerSupplyInfo};

use crate::battery::{BatteryError, BatteryErrorCodes};

use crate::configs::BaseConfig;
use crate::output_message::{Message, StdOut, BATTERY};

#[derive(Debug, Args)]
pub struct Battery {
    #[command(subcommand)]
    command: BatteryCommands,
}

#[derive(Debug, Subcommand)]
enum BatteryCommands {
    #[command(about = "Get battery info")]
    Info,
}

impl Battery {
    pub async fn execute(&self, config: &BaseConfig) -> Result<()> {
        match &self.command {
            BatteryCommands::Info => {
                let battery_path = config.interfaces.battery.device.clone();
                StdOut::info(&format!("Battery path : {}", battery_path), Some(BATTERY));
                let battery = Power {
                    path: "/path/of/battery".to_string(),
                    currnet_now: "/path/of/battery/current".to_string(),
                };

                let _ = match battery.info() {
                    Ok(power) => {
                        StdOut::info(&format!("Battery info : {:?}", power), Some(BATTERY));
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                        bail!(BatteryError::new(
                            BatteryErrorCodes::UnableToDetectBattery,
                            "unable to get battery info".to_string()
                        ))
                    }
                };

                Ok(())
            }
        }
    }
}
