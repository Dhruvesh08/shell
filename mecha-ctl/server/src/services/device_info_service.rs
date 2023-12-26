use anyhow::Result;
use mecha_device_info_ctl::DeviceInfoControl;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct DeviceInfoCtl {
    device_info: DeviceInfoControl,
}

#[allow(non_snake_case)]
pub mod deviceinfo {
    tonic::include_proto!("deviceinfo");
}

pub use deviceinfo::{
    device_info_ctl_service_server::{DeviceInfoCtlService, DeviceInfoCtlServiceServer},
    CpuInfoResponse, DiskInfoResponse, Empty, MemoryInfoResponse,
};

#[tonic::async_trait]
impl DeviceInfoCtlService for DeviceInfoCtl {
    async fn get_cpu_info(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<CpuInfoResponse>, Status> {
        println!("get_cpu_info");
        let deviceinfo = match self.device_info.get_cpu_info() {
            Ok(cpu_info) => cpu_info,
            Err(err) => return Err(Status::from_error(err.into())),
        };

        let response = CpuInfoResponse {
            cpu_info: Some(deviceinfo::CpuInfo {
                // Initialize with the CPU info obtained from the SDK
                cpu_name: deviceinfo.cpu_name,
                cpu_frequency: deviceinfo.cpu_frequency,
                number_of_cores: deviceinfo.number_of_cores as u32, // Make sure to convert usize to i32
            }),
        };

        Ok(Response::new(response))
    }

    async fn get_disk_info(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<DiskInfoResponse>, Status> {
        //print disk info
        println!("get_disk_info");
        let disk_info = match self.device_info.get_disk_info() {
            Ok(disk_info) => disk_info,
            Err(err) => return Err(Status::from_error(err.into())),
        };

        let response = DiskInfoResponse {
            disk_info: disk_info
                .into_iter()
                .map(|disk| deviceinfo::DiskInfo {
                    name: disk.name,
                    fs: disk.fs,
                    removable: disk.removable,
                    mount_point: disk.mount_point,
                    used_space: disk.used_space,
                    total_space: disk.total_space,
                })
                .collect(),
        };

        Ok(Response::new(response))
    }

    async fn get_memory_info(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<MemoryInfoResponse>, Status> {
        //print memory info
        println!("get_memory_info");
        match self.device_info.get_memory_info() {
            Ok(memory_info) => Ok(Response::new(MemoryInfoResponse {
                memory_info: Some(deviceinfo::MemoryInfo {
                    total_memory: memory_info.total_memory,
                    free_memory: memory_info.free_memory,
                    available_memory: memory_info.available_memory,
                }),
            })),
            Err(err) => Err(Status::from_error(err.into())),
        }
    }
}
