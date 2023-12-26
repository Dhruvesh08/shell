#![deny(clippy::all)]
mod led;
pub use led::{LedControl, LedColor};

mod errors;
pub use errors::{LedctlError, LedctlErrorCodes};