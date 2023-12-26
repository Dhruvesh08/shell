#![deny(clippy::all)]

use anyhow::Result;
use clap::{Args, Subcommand};

pub use crate::network::{NetworkError, NetworkErrorCodes};
use crate::output_message::{Message, StdOut, WIFI};
pub use mecha_network_ctl::wireless_network::WirelessNetworkControl;

#[derive(Debug, Args)]
#[command(name = "network")]
pub struct Network {
    #[command(subcommand)]
    command: NetworkCommand,
}

#[derive(Debug, Subcommand)]
enum NetworkCommand {
    #[command(about = "Scan for wireless networks")]
    Scan,

    #[command(about = "Add a wireless network")]
    Add(WirelessAddArgs),

    #[command(about = "Remove a wireless network")]
    Remove(WirelessRemoveArgs),

    #[command(about = "Connect to a wireless network")]
    Connect(WirelessConnectArgs),

    #[command(about = "Status of current network")]
    Status,
}

#[derive(Debug, Args)]
struct WirelessAddArgs {
    #[arg(required = true)]
    ssid: String,

    #[arg(required = true)]
    password: String,
}

#[derive(Debug, Args)]
struct WirelessRemoveArgs {
    #[arg(required = true)]
    ssid: String,
}

#[derive(Debug, Args)]
struct WirelessConnectArgs {
    #[arg(required = true)]
    ssid: String,

    #[arg(required = true)]
    password: String,
}

impl Network {
    pub async fn execute(&self) -> Result<()> {
        let network_module = WirelessNetworkControl::new();
        match &self.command {
            NetworkCommand::Scan => {
                let _scan_results = match network_module.scan_wireless_network().await {
                    Ok(scan_results) => {
                        scan_results.iter().for_each(|network| {
                            StdOut::info(
                                &format!(
                                    "Network SSID: {}, Signal Strength: {}, GHz: {}",
                                    network.name, network.signal, network.frequency
                                ),
                                Some(WIFI),
                            );
                        });
                        scan_results
                    }
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            NetworkCommand::Add(args) => {
                let ssid = &args.ssid;
                let psk = &args.password;
                let _add_wireless_network = match WirelessNetworkControl::connect_wireless_network(
                    ssid.as_str(),
                    psk.as_str(),
                )
                .await
                {
                    Ok(()) => (StdOut::success(&format!("Added network: {}", ssid))),
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            NetworkCommand::Remove(args) => {
                //take ssid form arg and convert it into i32
                let network_id = args.ssid.parse::<usize>().unwrap();

                // use args and use remove_wireless_network
                let _ = match WirelessNetworkControl::remove_wireless_network(network_id).await {
                    Ok(remove_results) => {
                        StdOut::success(&format!("Removed network: {:?}", remove_results));
                        remove_results
                    }
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            NetworkCommand::Connect(args) => {
                let _ = match WirelessNetworkControl::connect_wireless_network(
                    &args.ssid,
                    &args.password,
                )
                .await
                {
                    Ok(connect_results) => {
                        StdOut::success(&format!("Connected to network: {:?}", connect_results));
                        connect_results
                    }
                    Err(e) => {
                        return Err(e);
                    }
                };
            }

            NetworkCommand::Status => {
                let connected = WirelessNetworkControl::wireless_network_status().await;

                if connected {
                    StdOut::success("Wireless network is connected.");
                } else {
                    println!("Wireless network is not connected.");
                }
            }
        }

        Ok(())
    }
}
