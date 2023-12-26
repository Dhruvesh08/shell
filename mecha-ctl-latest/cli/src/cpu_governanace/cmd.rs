use anyhow::{bail, Result};
use clap::{Args, Subcommand};

use console::Emoji;
use mecha_cpu_governor_ctl::{
    CpuFrequency, CpuGovernanceCtl, CpuGovernanceCtlError, CpuGovernanceCtlErrorCodes,
};

use crate::output_message::{Message, StdOut};

#[derive(Debug, Args)]
pub struct CpuGoverner {
    #[command(subcommand)]
    command: CpuGovernerCommands,
}

#[derive(Debug, Subcommand)]
enum CpuGovernerCommands {
    #[command(about = "Get cpu frequency")]
    GetFrequency,
    #[command(about = "set cpu frequency")]
    SetFrequency(CpuFrequencyControl),
}

#[derive(Debug, Args)]
pub struct CpuFrequencyControl {
    #[arg(required = true, short = 'f')]
    frequency: u64,
}

impl CpuGoverner {
    pub async fn execute(&self) -> Result<()> {
        let cpu_governer_control = CpuGovernanceCtl::new();
        match &self.command {
            CpuGovernerCommands::GetFrequency => match cpu_governer_control.get_cpu_frequency() {
                Ok(frequency) => {
                    StdOut::info(&format!("Cpu frequncy : {}", frequency), None);
                }
                Err(e) => {
                    bail!(CpuGovernanceCtlError::new(
                        CpuGovernanceCtlErrorCodes::FailedToGetCpuFrequency,
                        format!("Error getting cpu frequency: {}", e)
                    ),)
                }
            },
            CpuGovernerCommands::SetFrequency(frequency) => {
                match cpu_governer_control.set_cpu_frequency(CpuFrequency::Freq1200000) {
                    Ok(_) => {
                        StdOut::success(
                            format!("Cpu frequncy set to : {}", frequency.frequency).as_str(),
                        );
                    }
                    Err(e) => {
                        bail!(CpuGovernanceCtlError::new(
                            CpuGovernanceCtlErrorCodes::FailedToSetCpuFrequency,
                            format!("Error setting cpu frequency: {}", e)
                        ),);
                    }
                }
            }
        }
        Ok(())
    }
}
