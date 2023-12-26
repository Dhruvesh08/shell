use anyhow::{bail, Result};
use clap::{Args, Subcommand};

use mecha_device_info_ctl::{DeviceInfoControl, DeviceInfoCtlError, DeviceInfoCtlErrorCodes};
use mecha_metrics_ctl::{DeviceMetricsCtl, DeviceMetricsCtlError, DeviceMetricsCtlErrorCodes};

use crate::output_message::{Message, StdOut, CPU, RAM, STORAGE};

#[derive(Debug, Args)]
pub struct DeviceInfo {
    #[command(subcommand)]
    command: DeviceInfoCommands,
}

#[derive(Debug, Subcommand)]
enum DeviceInfoCommands {
    Cpu(Cpu),
    Memory(Memory),
    Storage(Storage),
}

//create cpu args
#[derive(Debug, Args)]
pub struct Cpu {
    #[command(subcommand)]
    command: CpuCommands,
}

//create cpu subcommands
#[derive(Debug, Subcommand)]
enum CpuCommands {
    #[command(about = "Get cpu usage")]
    Usage,
    #[command(about = "Get cpu info")]
    Info,
}

#[derive(Debug, Args)]
pub struct Memory {
    #[command(subcommand)]
    command: MemoryCommands,
}

#[derive(Debug, Subcommand)]
enum MemoryCommands {
    #[command(about = "Get memory usage")]
    Usage,
    #[command(about = "Get memory info")]
    Info,
}

#[derive(Debug, Args)]
pub struct Storage {
    #[command(subcommand)]
    command: StorageCommands,
}

#[derive(Debug, Subcommand)]
enum StorageCommands {
    #[command(about = "Get storage usage")]
    Usage,

    #[command(about = "Get storage info")]
    Info,
}

impl DeviceInfo {
    pub async fn execute(&self) -> Result<()> {
        let device_info = DeviceInfoControl::new();
        let device_matrics = DeviceMetricsCtl::new();
        match &self.command {
            DeviceInfoCommands::Cpu(cpu) => {
                // Handle CPU commands
                match &cpu.command {
                    CpuCommands::Usage => match device_matrics.get_cpu_usage() {
                        Ok(usage) => {
                            StdOut::info(&format!("Usage : {}", usage), Some(CPU));
                        }
                        Err(e) => {
                            bail!(DeviceMetricsCtlError::new(
                                DeviceMetricsCtlErrorCodes::FailedToGetCpuUsage,
                                e.to_string(),
                            ),);
                        }
                    },
                    CpuCommands::Info => match device_info.get_cpu_info() {
                        Ok(info) => {
                            //convert info to string
                            let info = format!("{:?}", info);
                            StdOut::info(&format!("Info : {}", info), Some(CPU));
                        }
                        Err(e) => {
                            bail!(DeviceInfoCtlError::new(
                                DeviceInfoCtlErrorCodes::UnknownError,
                                "uable to get cpu info".to_string()
                            ))
                        }
                    },
                }
            }
            DeviceInfoCommands::Memory(memory) => {
                // Handle Memory commands
                match &memory.command {
                    MemoryCommands::Usage => match device_matrics.get_memory_usage() {
                        Ok(usage) => {
                            StdOut::info(&format!("Usage : {}", usage), Some(RAM));
                        }
                        Err(e) => {
                            bail!(DeviceMetricsCtlError::new(
                                DeviceMetricsCtlErrorCodes::FailedToGetCpuUsage,
                                e.to_string(),
                            ),);
                        }
                    },
                    MemoryCommands::Info => match device_info.get_memory_info() {
                        Ok(info) => {
                            let total_memory_gb = info.total_memory / 1073741824;
                            let available_memory_gb = info.available_memory / 1073741824;
                            let free_memory_gb = info.free_memory / 1073741824;
                            let formatted_info = format!(
                                "Total memory: {:.2} GB,\n    Available memory: {:.2} GB,\n    Free memory: {:.2} GB",
                                total_memory_gb, available_memory_gb,free_memory_gb
                            );

                            StdOut::info(&format!("Memory Info : {}", formatted_info), Some(RAM));
                        }
                        Err(e) => {
                            bail!(DeviceInfoCtlError::new(
                                DeviceInfoCtlErrorCodes::FailedToGetMemoryUsage,
                                e.to_string()
                            ))
                        }
                    },
                }
            }
            DeviceInfoCommands::Storage(storage) => {
                // Handle Storage commands
                match &storage.command {
                    StorageCommands::Usage => match device_matrics.get_disk_usage() {
                        Ok(usage) => {
                            //convert u64 to human readable format
                            let usage = format!("{:?}", usage/1073741824);
                            StdOut::info(&format!("Storage usage : {:.2} GB", usage), Some(STORAGE));
                        }
                        Err(e) => {
                            bail!(DeviceMetricsCtlError::new(
                                DeviceMetricsCtlErrorCodes::FailedToGetCpuUsage,
                                e.to_string(),
                            ),);
                        }
                    },
                    StorageCommands::Info => match device_info.get_disk_info() {
                        Ok(info) => {
                            StdOut::info(&format!("Storage info : {:?}", info), Some(STORAGE));
                        }
                        Err(e) => {
                            bail!(DeviceInfoCtlError::new(
                                DeviceInfoCtlErrorCodes::FailedToGetDiskUsage,
                                e.to_string()
                            ))
                        }
                    },
                }
            }
        }
        Ok(())
    }
}
