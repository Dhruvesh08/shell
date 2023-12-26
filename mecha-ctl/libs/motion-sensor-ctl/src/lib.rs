#![deny(clippy::all)]
mod motion_sensor;
pub use motion_sensor::MotionSensorControl;

mod errors;
pub use errors::{MotionSensorControlError, MotionSensorControlErrorCodes};
