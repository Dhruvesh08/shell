use anyhow::{bail, Result};
use std::fs::{read_to_string, File};
use std::io::Write;
use tracing::{error as trace_error, info, instrument, trace, warn};

use crate::{CpuGovernanceCtlError, CpuGovernanceCtlErrorCodes};

#[derive(Debug)]
pub enum CpuFrequency {
    Freq1200000,
    Freq1600000,
    Freq1800000,
}

#[derive(Debug)]
pub struct CpuGovernanceCtl {
    pub cpu_frequency_path: String,
}

impl CpuGovernanceCtl {
    pub fn new() -> Self {
        trace!(tack = "CpuGovernanceCtl instace", "init");
        CpuGovernanceCtl {
            cpu_frequency_path: String::from("/sys/devices/system/cpu/cpu0/cpufreq"),
        }
    }

    #[instrument(skip(self))]
    pub fn set_cpu_governor(&self) -> Result<()> {
        trace!(task = "set_cpu_governor", "init");
        let mut file = match File::create(format!("{}/scaling_governor", self.cpu_frequency_path)) {
            Ok(file) => {
                info!(task = "set_cpu_governor", "set cpu governor to userspace");
                file
            }
            Err(e) => bail!(CpuGovernanceCtlError::new(
                CpuGovernanceCtlErrorCodes::FailedToSetCpuGovernorPath,
                format!("failed to set CPU governor: {}", e)
            )),
        };
        match file.write_all(b"userspace") {
            Ok(_) => {
                info!(task = "set_cpu_governor", "set cpu governor to userspace");
                Ok(())
            }
            Err(e) => bail!(CpuGovernanceCtlError::new(
                CpuGovernanceCtlErrorCodes::FailedToSetCpuGovernor,
                format!("failed to set CPU governor: {}", e)
            )),
        }
    }

    #[instrument(skip(self))]
    pub fn get_cpu_governor(&self) -> Result<String> {
        match read_to_string(format!("{}/scaling_governor", self.cpu_frequency_path)) {
            Ok(content) => {
                info!(task = "get_cpu_governor", "get cpu governor: {}", content);
                Ok(content)
            }
            Err(e) => {
                trace_error!(
                    task = "get_cpu_governor",
                    "failed to get CPU governor: {}",
                    e
                );
                bail!(CpuGovernanceCtlError::new(
                    CpuGovernanceCtlErrorCodes::FailedToGetCpuGovernor,
                    format!("failed to get CPU governor: {}", e)
                ))
            }
        }
    }

    #[instrument(skip(self))]
    pub fn get_cpu_frequency(&self) -> Result<String> {
        match read_to_string(format!("{}/scaling_cur_freq", self.cpu_frequency_path)) {
            Ok(content) => Ok(content),
            Err(e) => {
                trace_error!(
                    task = "get_cpu_frequency",
                    "failed to get CPU frequency: {}",
                    e
                );
                bail!(CpuGovernanceCtlError::new(
                    CpuGovernanceCtlErrorCodes::FailedToGetCpuFrequency,
                    format!("failed to get CPU frequency: {}", e)
                ))
            }
        }
    }

    #[instrument(skip(self))]
    pub fn set_cpu_frequency(&self, frequency: CpuFrequency) -> Result<()> {
        let freq_str = match frequency {
            CpuFrequency::Freq1200000 => "1200000",
            CpuFrequency::Freq1600000 => "1600000",
            CpuFrequency::Freq1800000 => "1800000",
        };

        let mut file = match File::create(format!("{}/scaling_setspeed", self.cpu_frequency_path)) {
            Ok(file) => {
                info!(
                    task = "set_cpu_frequency",
                    "set cpu frequency to {}", freq_str
                );
                file
            }
            Err(e) => {
                trace_error!(
                    task = "set_cpu_frequency",
                    "failed to set CPU frequency: {}",
                    e
                );
                bail!(CpuGovernanceCtlError::new(
                    CpuGovernanceCtlErrorCodes::FailedToSetCpuFrequencyPath,
                    format!("failed to set CPU frequency: {}", e)
                ))
            }
        };
        match file.write_all(freq_str.as_bytes()) {
            Ok(_) => {
                info!(
                    task = "set_cpu_frequency",
                    "set cpu frequency to {}", freq_str
                );
                Ok(())
            }
            Err(e) => {
                warn!(
                    task = "set_cpu_frequency",
                    "failed to set CPU frequency: {}", e
                );
                bail!(CpuGovernanceCtlError::new(
                    CpuGovernanceCtlErrorCodes::FailedToSetCpuFrequency,
                    format!("failed to set CPU frequency: {}", e)
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use tempfile::tempdir;
    #[test]
    fn test_cpu_ctrl() {
        let cpu_ctrl = CpuGovernanceCtl::new();
        assert_eq!(
            cpu_ctrl.cpu_frequency_path,
            "/sys/devices/system/cpu/cpu0/cpufreq"
        );
    }

    #[test]
    fn test_set_cpu_governor() {
        let dir = tempdir().unwrap();
        let cpu_frequency_path = dir.path().join("cpu_frequency_path");
        fs::create_dir(&cpu_frequency_path).unwrap();

        let cpu_ctrl = CpuGovernanceCtl {
            cpu_frequency_path: cpu_frequency_path.to_str().unwrap().to_string(),
        };

        // Test set_cpu_governor
        let result = cpu_ctrl.set_cpu_governor();
        assert!(result.is_ok());

        // Check that the file was written correctly
        let scaling_governor_path = cpu_frequency_path.join("scaling_governor");
        assert!(scaling_governor_path.exists());
        let contents = fs::read_to_string(&scaling_governor_path).unwrap();
        assert_eq!(contents, "userspace");
    }

    #[test]
    fn test_get_cpu_governor() {
        let dir = tempdir().unwrap();
        let cpu_frequency_path = dir.path().join("cpu_frequency_path");
        fs::create_dir(&cpu_frequency_path).unwrap();

        let cpu_ctrl = CpuGovernanceCtl {
            cpu_frequency_path: cpu_frequency_path.to_str().unwrap().to_string(),
        };

        // Write a test value to the scaling_governor file
        let mut file = File::create(cpu_frequency_path.join("scaling_governor")).unwrap();
        file.write_all(b"userspace").unwrap();

        // Test get_cpu_governor
        let result = cpu_ctrl.get_cpu_governor();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "userspace");
    }

    #[test]
    fn test_get_cpu_frequency() {
        let dir = tempdir().unwrap();
        let cpu_frequency_path = dir.path().join("cpu_frequency_path");
        fs::create_dir(&cpu_frequency_path).unwrap();

        let cpu_ctrl = CpuGovernanceCtl {
            cpu_frequency_path: cpu_frequency_path.to_str().unwrap().to_string(),
        };

        // Write a test value to the scaling_cur_freq file
        let mut file = File::create(cpu_frequency_path.join("scaling_cur_freq")).unwrap();
        file.write_all(b"1200000").unwrap();

        // Test get_cpu_frequency
        let result = cpu_ctrl.get_cpu_frequency();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1200000");
    }

    #[test]
    fn test_set_cpu_frequency() {
        let dir = tempdir().unwrap();
        let cpu_frequency_path = dir.path().join("cpu_frequency_path");
        fs::create_dir(&cpu_frequency_path).unwrap();

        let cpu_ctrl = CpuGovernanceCtl {
            cpu_frequency_path: cpu_frequency_path.to_str().unwrap().to_string(),
        };

        // Test set_cpu_frequency
        let result = cpu_ctrl.set_cpu_frequency(CpuFrequency::Freq1200000);
        assert!(result.is_ok());

        // Check that the file was written correctly
        let scaling_setspeed_path = cpu_frequency_path.join("scaling_setspeed");
        assert!(scaling_setspeed_path.exists());
        let contents = fs::read_to_string(&scaling_setspeed_path).unwrap();
        assert_eq!(contents, "1200000");
    }
}
