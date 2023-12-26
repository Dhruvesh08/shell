#![deny(clippy::all)]

mod cpu;
pub use cpu::{CpuFrequency, CpuGovernanceCtl};

mod errors;
pub use errors::{CpuGovernanceCtlError, CpuGovernanceCtlErrorCodes};
