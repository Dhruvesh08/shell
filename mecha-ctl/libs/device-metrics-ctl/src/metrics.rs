use crate::errors::{DeviceMetricsCtlError, DeviceMetricsCtlErrorCodes};
use anyhow::{bail, Result};
use sysinfo::{CpuExt, DiskExt, System, SystemExt};
use tracing::{error as trace_error, info, trace, warn,instrument};

#[derive(Debug, Default)]
pub struct DeviceMetricsCtl {
    system: System,
}

impl DeviceMetricsCtl {
    pub fn new() -> Self {
        trace!(task = "device_metrics instance", "init");
        let mut system = System::new_all();
        system.refresh_all();
        DeviceMetricsCtl { system }
    }

    #[instrument(skip(self))]
    pub fn get_cpu_usage(&self) -> Result<f32> {
        trace!(task = "get_cpu_usage", "init");
        match self.system.global_cpu_info().cpu_usage() {
            cpu_usage => {
                info!(task = "get_cpu_usage", "cpu usage: {}", cpu_usage);
                Ok(cpu_usage)
            }

            _ => {
                trace_error!(task = "get_cpu_usage", "failed to get CPU usage");
                bail!(DeviceMetricsCtlError::new(
                    DeviceMetricsCtlErrorCodes::FailedToGetCpuUsage,
                    "failed to get CPU usage".to_string(),
                ))
            }
        }
    }

    #[instrument(skip(self))]
    pub fn get_memory_usage(&self) -> Result<u64> {
        trace!(task = "get_memory_usage", "init");
        match self.system.used_memory() {
            memory_usage => {
                info!(task = "get_memory_usage", "memory usage: {}", memory_usage);
                Ok(memory_usage)
            }
            _ => {
                trace_error!(task = "get_memory_usage", "failed to get memory usage");
                bail!(DeviceMetricsCtlError::new(
                    DeviceMetricsCtlErrorCodes::FailedToGetMemoryUsage,
                    "failed to get memory usage".to_string(),
                ))
            }
        }
    }

    #[instrument(skip(self))]
    pub fn get_disk_usage(&self) -> Result<u64> {
        trace!(task = "get_disk_usage", "init");
        //take primary disk
        match self.system.disks().iter().take(1).next() {
            Some(primary_disk) => {
                info!(
                    task = "get_disk_usage",
                    "disk usage: {}",
                    primary_disk.total_space() - primary_disk.available_space()
                );
                let disk_usage = primary_disk.total_space() - primary_disk.available_space();
                Ok(disk_usage)
            }
            None => {
                warn!(task = "get_disk_usage", "failed to get disk usage");
                bail!(DeviceMetricsCtlError::new(
                    DeviceMetricsCtlErrorCodes::FailedToGetDiskUsage,
                    "failed to get disk usage".to_string(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::{automock, predicate::*};

    #[automock]
    trait DeviceMetricsCtlTrait {
        fn get_cpu_usage(&self) -> Result<f32>;
        fn get_memory_usage(&self) -> Result<u64>;
        fn get_disk_usage(&self) -> Result<u64>;
    }

    #[test]
    fn test_device_metrics() {
        let mut device_metrics = MockDeviceMetricsCtlTrait::new();
        device_metrics.expect_get_cpu_usage().returning(|| Ok(0.0));
        device_metrics.expect_get_memory_usage().returning(|| Ok(0));
        device_metrics.expect_get_disk_usage().returning(|| Ok(0));

        let cpu_usage = device_metrics.get_cpu_usage();
        assert!(cpu_usage.is_ok());
        assert_eq!(cpu_usage.unwrap(), 0.0);

        let memory_usage = device_metrics.get_memory_usage();
        assert!(memory_usage.is_ok());
        assert_eq!(memory_usage.unwrap(), 0);

        let disk_usage = device_metrics.get_disk_usage();
        assert!(disk_usage.is_ok());
        assert_eq!(disk_usage.unwrap(), 0);
    }
}
