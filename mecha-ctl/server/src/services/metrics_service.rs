use anyhow::Result;
pub use mecha_metrics_ctl::DeviceMetricsCtl;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct DeviceMetricsService {
    pub metrics: DeviceMetricsCtl,
}

#[allow(non_snake_case)]
pub mod metrics {
    tonic::include_proto!("metrics");
}

pub use metrics::{
    metrics_service_server::{MetricsService, MetricsServiceServer},
    Empty, GetCpuUsageResponse, GetDiskUsageResponse, GetMemoryUsageResponse,
};

#[tonic::async_trait]
impl MetricsService for DeviceMetricsService {
    async fn get_cpu_usage(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<GetCpuUsageResponse>, Status> {
        let cpu_usage = match self.metrics.get_cpu_usage() {
            Ok(cpu_usage) => cpu_usage,
            Err(err) => return Err(Status::from_error(err.into())),
        };

        let response = GetCpuUsageResponse {
            cpu_usage: cpu_usage,
        };

        Ok(Response::new(response))
    }

    async fn get_memory_usage(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<GetMemoryUsageResponse>, Status> {
        let memory_usage = match self.metrics.get_memory_usage() {
            Ok(memory_usage) => memory_usage,
            Err(err) => return Err(Status::from_error(err.into())),
        };

        let response = GetMemoryUsageResponse {
            memory_usage: memory_usage,
        };

        Ok(Response::new(response))
    }

    async fn get_disk_usage(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<GetDiskUsageResponse>, Status> {
        let disk_usage = match self.metrics.get_disk_usage() {
            Ok(disk_usage) => disk_usage,
            Err(err) => return Err(Status::from_error(err.into())),
        };

        let response = GetDiskUsageResponse {
            disk_usage: disk_usage,
        };

        Ok(Response::new(response))
    }
}
