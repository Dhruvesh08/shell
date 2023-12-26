use crate::errors::{BluetoothError, BluetoothErrorCodes};
use anyhow::{bail, Result};
use bluer::Session;
use tracing::{error as trace_error, info, trace};
//allow dead_code
#[allow(dead_code)]

pub struct BluetoothControl {
    session: Session,
}

impl BluetoothControl {
    pub async fn new() -> Result<Self> {
        let session = Session::new().await?;
        Ok(Self { session })
    }

    pub async fn bluetooth_status(&self) -> Result<bool> {
        trace!(task = "bluetooth_status", "init");
        let adapter = self.session.default_adapter().await?;
        let powered = match adapter.is_powered().await {
            Ok(powered) => powered,
            Err(e) => {
                trace_error!(
                    task = "bluetooth_status",
                    "unable to get bluetooth status: {}",
                    e
                );
                bail!(BluetoothError::new(
                    BluetoothErrorCodes::UnableToGetBluetoothDeviceStatus,
                    format!("unable to get bluetooth status: {}", e),
                ))
            }
        };
        Ok(powered)
    }

    pub async fn enable_bluetooth(&self) -> Result<()> {
        trace!(task = "enable_bluetooth", "init");
        let adapter = self.session.default_adapter().await?;
        match adapter.set_powered(true).await {
            Ok(_) => {
                info!(task = "enable_bluetooth", "bluetooth turned on");
                Ok(())
            }
            Err(e) => {
                trace_error!(
                    task = "enable_bluetooth",
                    "unable to turn on bluetooth: {}",
                    e
                );
                bail!(BluetoothError::new(
                    BluetoothErrorCodes::UnableToTurnOnBluetooth,
                    format!("unable to turn on bluetooth: {}", e),
                ))
            }
        }
    }

    pub async fn disable_bluetooth(&self) -> Result<()> {
        trace!(task = "disable_bluetooth", "init");
        let adapter = self.session.default_adapter().await?;
        match adapter.set_powered(false).await {
            Ok(_) => {
                info!(task = "disable_bluetooth", "bluetooth turned off");
                Ok(())
            }
            Err(e) => {
                trace_error!(
                    task = "disable_bluetooth",
                    "unable to turn off bluetooth: {}",
                    e
                );
                bail!(BluetoothError::new(
                    BluetoothErrorCodes::UnableToTurnOffBluetooth,
                    format!("unable to turn off bluetooth: {}", e),
                ))
            }
        }
    }
}


// //unit tests
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use bluer::adapter::Adapter;
//     use bluer::adapter::AdapterState;
//     use bluer::adapter::BluetoothVersion;
//     use bluer::adapter::Device;
//     use bluer::adapter::DeviceType;
//     use bluer::adapter::DiscoveryFilter;
//     use bluer::adapter::DiscoverySession;
//     use bluer::adapter::Session;
//     use bluer::adapter::Transport;
//     use bluer::Error;
//     use bluer::ErrorKind;
//     use bluer::Session as BluerSession;
//     use std::sync::Arc;
//     use tokio::sync::Mutex;

//     #[derive(Clone)]
//     struct MockSession {
//         adapter: Arc<Mutex<MockAdapter>>,
//     }

//     impl MockSession {
//         fn new(adapter: MockAdapter) -> Self {
//             Self {
//                 adapter: Arc::new(Mutex::new(adapter)),
//             }
//         }
//     }

//     impl BluerSession for MockSession {
//         fn default_adapter(&self) -> Box<dyn Adapter> {
//             Box::new(self.adapter.clone())
//         }
//     }

//     #[derive(Clone)]
//     struct MockAdapter {
//         powered: bool,
//     }

//     impl MockAdapter {
//         fn new(powered: bool) -> Self {
//             Self { powered }
//         }
//     }

//     #[async_trait]
//     impl Adapter for MockAdapter {
//         async fn address(&self) -> Result<String, Error> {
//             Ok("mock_address".to_string())
//         }

//         async fn name(&self) -> Result<String, Error> {
//             Ok("mock_name".to_string())
//         }

//         async fn alias(&self) -> Result<String, Error> {
//             Ok("mock_alias".to_string())
//         }

//         async fn set_alias(&self, _alias: &str) -> Result<(), Error> {
//             Ok(())
//         }

//         async fn class(&self) -> Result<u32, Error> {
//             Ok(0)
//         }

//         async fn set_class(&self, _class: u32) -> Result<(), Error> {
//             Ok(())
//         }

//         async fn powered(&self) -> Result<bool, Error> {
//             Ok(self.powered)
//         }

//         async fn set_powered(&self, powered: bool) -> Result<(), Error> {
//             Ok(())
//         }

//         async fn discoverable(&self) -> Result<bool, Error> {
//             Ok(false)
//         }

//     }
// }