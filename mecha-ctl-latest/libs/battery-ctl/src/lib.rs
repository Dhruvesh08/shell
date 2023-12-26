#![deny(clippy::all)]
mod power_supply;
pub use power_supply::{Battery, BatteryControl, PowerSupplyInfo};

mod errors;
pub use errors::{PowerSupplyError, PowerSupplyErrorCodes};
