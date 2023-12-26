#![deny(clippy::all)]

mod device_info;
pub use device_info::DeviceInfoControl;

mod errors;
pub use errors::{DeviceInfoCtlError, DeviceInfoCtlErrorCodes};
