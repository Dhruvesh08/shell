use crate::errors::{LedctlError, LedctlErrorCodes};
use anyhow::{bail, Result};
use std::fs::File;
use std::io::{self, Write};
use tracing::{error as trace_error, info, trace,instrument};

#[derive(Debug, PartialEq, Eq)]
pub enum LedColor {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Default)]
pub struct LedControl {
    red_led_path: String,
    green_led_path: String,
    blue_led_path: String,
}

impl LedControl {
    // Constructor for LedControl
    pub fn new(red_led_path: &str, green_led_path: &str, blue_led_path: &str) -> Self {
        trace!(task = "led_ctrl instance", "init");
        LedControl {
            red_led_path: String::from(red_led_path),
            green_led_path: String::from(green_led_path),
            blue_led_path: String::from(blue_led_path),
        }
    }

    // Function to set the LED based on the specified color
    #[instrument(skip(self))]
    pub fn set_led(&self, red: u8, green: u8, blue: u8) -> Result<()> {
        trace!(task = "set_led", "init");
    
        // Set red LED
        if let Err(e) = self.write_brightness(&self.red_led_path, &red.to_string()) {
            trace_error!(task = "set_led", "unable to write brightness value: {}", e);
            bail!(LedctlError::new(
                LedctlErrorCodes::InvalidLedPathValueError,
                format!("unable to write brightness value: {}", e),
            ));
        }
    
        // Set green LED
        if let Err(e) = self.write_brightness(&self.green_led_path, &green.to_string()) {
            trace_error!(task = "set_led", "unable to write brightness value: {}", e);
            bail!(LedctlError::new(
                LedctlErrorCodes::InvalidLedPathValueError,
                format!("unable to write brightness value: {}", e),
            ));
        }
    
        // Set blue LED
        if let Err(e) = self.write_brightness(&self.blue_led_path, &blue.to_string()) {
            trace_error!(task = "set_led", "unable to write brightness value: {}", e);
            bail!(LedctlError::new(
                LedctlErrorCodes::InvalidLedPathValueError,
                format!("unable to write brightness value: {}", e),
            ));
        }
    
        info!(task = "set_led", "set led to red: {}, green: {}, blue: {}", red, green, blue);
    
        Ok(())
    }

    // Function to clear the LED (set brightness to 0) based on the specified color
    #[instrument(skip(self))]
    pub fn clear_led(&self, color: LedColor) -> Result<()> {
        trace!(task = "clear_led", "init");
        let path = match color {
            LedColor::Red => &self.red_led_path,
            LedColor::Green => &self.green_led_path,
            LedColor::Blue => &self.blue_led_path,
        };

        //try to write the brightness value to the file or return an error
        if let Err(e) = self.write_brightness(path, "0") {
            trace_error!(
                task = "clear_led",
                "unable to write brightness value: {}",
                e
            );
            bail!(LedctlError::new(
                LedctlErrorCodes::InvalidLedPathValueError,
                format!("unable to read brightness value: {}", e),
            ));
        }
        info!(task = "clear_led", "clear led {:?}", color);
        Ok(())
    }

    // Private function to write to the brightness file with error handling
    fn write_brightness(&self, path: &str, value: &str) -> Result<(), io::Error> {
        trace!(task = "write_brightness", "init");
        let mut file = File::create(path)?;
        file.write_all(value.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempdir::TempDir;

    #[test]
    fn test_led_ctrl_new() {
        let red_led_path = "/sys/class/leds/red_led/brightness";
        let green_led_path = "/sys/class/leds/green_led/brightness";
        let blue_led_path = "/sys/class/leds/blue_led/brightness";
        let led_ctrl = LedControl::new(red_led_path, green_led_path, blue_led_path);
        assert_eq!(led_ctrl.red_led_path, red_led_path);
        assert_eq!(led_ctrl.green_led_path, green_led_path);
        assert_eq!(led_ctrl.blue_led_path, blue_led_path);
    }

    #[test]
    fn test_set_led() {
        let tmp_dir = TempDir::new("led").unwrap();

        let red_path = tmp_dir.path().join("red");
        let green_path = tmp_dir.path().join("green");
        let blue_path = tmp_dir.path().join("blue");

        File::create(red_path.clone()).unwrap();

        let led_ctrl = LedControl::new(
            red_path.to_str().unwrap(),
            green_path.to_str().unwrap(),
            blue_path.to_str().unwrap(),
        );

        let result = led_ctrl.set_led(1, 2, 3);
        assert!(result.is_ok());
    }

    #[test]
    fn test_clear_led() {
        let tmp_dir = TempDir::new("led").unwrap();

        let red_path = tmp_dir.path().join("red");
        File::create(red_path.clone()).unwrap();

        let led_ctrl = LedControl::new(red_path.clone().to_str().unwrap(), "green", "blue");

        // Write value
        std::fs::write(red_path.clone(), "1").unwrap();

        // Clear
        led_ctrl.clear_led(LedColor::Red).unwrap();

        // Check cleared
        assert_eq!(std::fs::read_to_string(red_path).unwrap(), "0");
    }
}
