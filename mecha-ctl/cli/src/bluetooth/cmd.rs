#![deny(clippy::all)]
use anyhow::{bail, Result};
use clap::{Args, Subcommand};

use crate::bluetooth::{BluetoothError, BluetoothErrorCodes};
pub use mecha_bluetooth_ctl::{BluetoothControl, BluetoothErrorCodes as BluetoothSDKErrorCode};

use crate::output_message::{Message, StdOut, BLUETOOTH, CONNECTION, DISCONNECT};

#[derive(Debug, Args)]
#[command(name = "bluetooth")]
pub struct Bluetooth {
    #[command(subcommand)]
    command: BluetoothCommand,
}

#[derive(Debug, Subcommand)]
enum BluetoothCommand {
    #[command(about = "Scan for bluetooth devices")]
    Scan,

    #[command(about = "Connect to a bluetooth device")]
    Connect(BluetoothConnectArgs),

    #[command(about = "Disconnect from a bluetooth device")]
    Disconnect(BluetoothDisconnectArgs),

    //status of bluetooth
    #[command(about = "Get the status of bluetooth")]
    Status,

    //turn on bluetooth
    #[command(about = "Turn on bluetooth")]
    On,

    //turn off bluetooth
    #[command(about = "Turn off bluetooth")]
    Off,
}

#[derive(Debug, Args)]
struct BluetoothConnectArgs {
    #[arg(required = true)]
    address: String,
}

#[derive(Debug, Args)]
struct BluetoothDisconnectArgs {
    #[arg(required = true)]
    address: String,
}

impl Bluetooth {
    pub async fn execute(&self) -> Result<()> {
        let controller = match BluetoothControl::new().await {
            Ok(controller) => controller,
            Err(err) => {
                println!("Error: {}", err);
                bail!(BluetoothError::new(
                    BluetoothErrorCodes::UnableToDetectBluetooth,
                    "unable to detect bluetooth".to_string()
                ))
            }
        };
        match &self.command {
            BluetoothCommand::Scan => {
                StdOut::info(&format!("Bluetooth  Scan"), Some(BLUETOOTH));
            }
            BluetoothCommand::Connect(args) => {
                StdOut::info(
                    &format!("Bluetooth  Connected to {}", args.address),
                    Some(CONNECTION),
                );
            }
            BluetoothCommand::Disconnect(args) => {
                StdOut::info(
                    &format!("Bluetooth  Disconnect {}", args.address),
                    Some(DISCONNECT),
                );
            }
            BluetoothCommand::Status => match controller.bluetooth_status().await {
                Ok(status) => {
                    StdOut::info(&format!("Bluetooth  status: {}", status), Some(BLUETOOTH));
                }
                Err(e) => {
                    bail!(BluetoothError::new(
                        BluetoothErrorCodes::UnableToDetectBluetooth,
                        "unable to detect bluetooth".to_string()
                    ))
                }
            },

            BluetoothCommand::On => match controller.enable_bluetooth().await {
                Ok(_) => {
                    StdOut::success("Bluetooth turned on");
                }
                Err(e) => {
                    bail!(BluetoothError::new(
                        BluetoothErrorCodes::UnableToConnectBluetooth,
                        "unable to connect bluetooth".to_string()
                    ))
                }
            },

            BluetoothCommand::Off => match controller.disable_bluetooth().await {
                Ok(_) => {
                    StdOut::success("Bluetooth turned off");
                }
                Err(e) => {
                    bail!(BluetoothError::new(
                        BluetoothErrorCodes::UnableToConnectBluetooth,
                        "unable to connect bluetooth".to_string()
                    ))
                }
            },
        }
        Ok(())
    }
}
