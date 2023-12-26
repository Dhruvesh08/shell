use anyhow::Result;
use mecha_cpu_governor_ctl::{CpuGovernanceCtl, CpuFrequency};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct CpuCtlService {
    pub cpu_ctrl_manager: CpuGovernanceCtl,
}

#[allow(non_snake_case)]
pub mod cpu_governor_ctrl {
    tonic::include_proto!("cpugovernorctl");
}

pub use cpu_governor_ctrl::{
    cpu_governor_ctl_service_server::{CpuGovernorCtlService, CpuGovernorCtlServiceServer},
    CpuFrequencyRequest, CpuFrequencyResponse, Empty, GovernorRequest, GovernorResponse,
};

#[tonic::async_trait]
impl CpuGovernorCtlService for CpuCtlService {
    async fn get_governor(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<GovernorResponse>, Status> {
        let governor = match self.cpu_ctrl_manager.get_cpu_governor() {
            Ok(governor) => GovernorResponse { result: governor },
            Err(err) => return Err(Status::from_error(err.into())),
        };

        Ok(Response::new(governor))
    }

    async fn set_governor(
        &self,
        request: Request<GovernorRequest>,
    ) -> Result<Response<Empty>, Status> {
        let _governor = request.into_inner().governor.to_string();
        match self.cpu_ctrl_manager.set_cpu_governor() {
            Ok(_) => Ok(Response::new(Empty {})),
            Err(err) => Err(Status::from_error(err.into())),
        }
    }

    async fn get_cpu_frequency(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<CpuFrequencyResponse>, Status> {
        let cpu_frequency = match self.cpu_ctrl_manager.get_cpu_frequency() {
            Ok(cpu_frequency) => CpuFrequencyResponse {
                result: cpu_frequency,
            },
            Err(err) => return Err(Status::from_error(err.into())),
        };

        Ok(Response::new(cpu_frequency))
    }

    async fn set_cpu_frequency(
        &self,
        request: Request<CpuFrequencyRequest>,
    ) -> Result<Response<Empty>, Status> {
        let cpu_frequency_str = request.into_inner().frequency;

        // Convert the CPU frequency string to an enum value.
        let cpu_frequency = match cpu_frequency_str.to_string().as_str() {
            "1200" => CpuFrequency::Freq1200000,
            "1600" => CpuFrequency::Freq1600000,
            "1800" => CpuFrequency::Freq1800000,
            _ => {
                return Err(Status::invalid_argument("Invalid CPU frequency value"));
            }
        };

        match self.cpu_ctrl_manager.set_cpu_frequency(cpu_frequency) {
            Ok(_) => Ok(Response::new(Empty {})),
            Err(err) => Err(Status::from_error(err.into())),
        }
    }
}
