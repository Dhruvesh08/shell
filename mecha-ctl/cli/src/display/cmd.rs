use anyhow::{bail, Result};
use clap::{Args, Subcommand};
pub use mecha_display_ctl::DisplayControl;
use tracing_subscriber::field::display;

use crate::configs::BaseConfig;
use crate::display::{DisplayError, DisplayErrorCodes};

use crate::output_message::{Message, StdOut, BRIGHTNESS, DISPLAY};

#[derive(Debug, Args)]
pub struct Display {
    #[command(subcommand)]
    command: DisplayCommands,
}

#[derive(Debug, Subcommand)]
enum DisplayCommands {
    #[command(about = "Get Display brightness")]
    GetBrightness,
    #[command(about = "Set Display brightness")]
    SetBrightness { brightness: u8 },
}

impl Display {
    pub async fn execute(&self, config: &BaseConfig) -> Result<()> {
        let display_path = config.interfaces.display.device.clone();

        // Use match to handle errors when creating a new DisplayControl instance
        let display = match DisplayControl::new(&display_path) {
            Ok(display) => display,
            Err(err) => {
                println!("Error: {}", err);
                bail!(DisplayError::new(
                    DisplayErrorCodes::Unknown,
                    "unable to get display".to_string()
                ))
            }
        };

        match &self.command {
            DisplayCommands::GetBrightness => {
                let brightness = match display.get_display_brightness() {
                    Ok(brightness) => brightness,
                    Err(err) => {
                        println!("Error: {}", err);
                        bail!(DisplayError::new(
                            DisplayErrorCodes::UnableToGetBrightness,
                            "unable to get display brightness".to_string()
                        ))
                    }
                };

                StdOut::info(
                    &format!("Current display brightness {}", brightness),
                    Some(DISPLAY),
                );
                Ok(())
            }
            DisplayCommands::SetBrightness { brightness } => {
                match display.set_display_brightness(*brightness) {
                    Ok(_) => {
                        StdOut::info(
                            &format!("Display brightbess set to {}", brightness),
                            Some(BRIGHTNESS),
                        );
                        Ok(())
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                        bail!(DisplayError::new(
                            DisplayErrorCodes::UnableToSetBrightness,
                            "unable to set display".to_string()
                        ))
                    }
                }
            }
        }
    }
}
