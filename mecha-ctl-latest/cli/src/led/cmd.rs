use anyhow::{bail, Result};
use clap::{Args, Subcommand};

// use crate::led::led_interface::LedManagerClient;
pub use mecha_led_ctl::{LedControl, LedctlError, LedctlErrorCodes};

use crate::output_message::{Message, StdOut, LED_COLOR, LIGHT_OFF};

use crate::configs::BaseConfig;

//create led args
#[derive(Debug, Args)]
pub struct Led {
    #[command(subcommand)]
    command: LedCommands,
}

//create led subcommands
#[derive(Debug, Subcommand)]
enum LedCommands {
    #[command(about = "Set Led")]
    SetLed(LedArgs),
    #[command(about = "Clear Led")]
    ClearLed(LedArgs),
}

#[derive(Debug, Args)]
struct LedArgs {
    #[arg(required = true)]
    rgb: String,
}

impl Led {
    pub async fn execute(&self, config: &BaseConfig) -> Result<()> {
        //device led path
        let red_led_path = config.interfaces.led.blue_led.clone();
        let green_led_path = config.interfaces.led.red_led.clone();
        let blue_led_path = config.interfaces.led.green_led.clone();

        let led = LedControl::new(&red_led_path, &green_led_path, &blue_led_path);

        match &self.command {
            LedCommands::SetLed(args) => {
                let rgb_values: Vec<&str> = args.rgb.split(',').collect();
                if rgb_values.len() != 3 {
                    return Err(anyhow::anyhow!("Invalid RGB values"));
                }
                let red = rgb_values[0].parse::<u8>()? != 0;
                let green = rgb_values[1].parse::<u8>()? != 0;
                let blue = rgb_values[2].parse::<u8>()? != 0;

                let _ = match led.set_led(red as u8, green as u8, blue as u8) {
                    Ok(_) => {
                        StdOut::info("Led set", Some(LED_COLOR));
                    }
                    Err(e) => {
                        bail!(LedctlError::new(
                            LedctlErrorCodes::InvalidLedPathValueError,
                            e.to_string(),
                        ),);
                    }
                };
            }
            LedCommands::ClearLed(args) => {
                let rgb_values: Vec<&str> = args.rgb.split(',').collect();
                if rgb_values.len() != 3 {
                    return Err(anyhow::anyhow!("Invalid RGB values"));
                }
                let red = rgb_values[0].parse::<u8>()? != 0;
                let green = rgb_values[1].parse::<u8>()? != 0;
                let blue = rgb_values[2].parse::<u8>()? != 0;

                let _ = match led.set_led(red as u8, green as u8, blue as u8) {
                    Ok(_) => {
                        StdOut::info("Led cleared", Some(LIGHT_OFF));
                    }
                    Err(e) => {
                        bail!(LedctlError::new(
                            LedctlErrorCodes::InvalidLedColorError,
                            e.to_string(),
                        ),);
                    }
                };
            }
        }
        Ok(())
    }
}
