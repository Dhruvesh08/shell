use anyhow::Result;
use mecha_bluetooth_ctl::BluetoothControl;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct Bluetooth {}

#[allow(non_snake_case)]
pub mod bluetooth {
    tonic::include_proto!("bluetooth");
}

pub use bluetooth::{
    bluetooth_service_server::{BluetoothService,BluetoothServiceServer}, BluetoothStatus, Empty, EmptyResponse,
};

#[tonic::async_trait]
impl BluetoothService for Bluetooth {
    async fn get_bluetooth_status(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<BluetoothStatus>, Status> {
        //try to crearte a new bluetooth controller or return an error using match
        let controller = match BluetoothControl::new().await {
            Ok(controller) => controller,
            Err(e) => {
                return Err(Status::from_error(e.into()));
            }
        };

        //try to get the bluetooth status or return an error using match
        let status = match controller.bluetooth_status().await {
            Ok(status) => status,
            Err(e) => {
                return Err(Status::from_error(e.into()));
            }
        };

        //return the bluetooth status
        Ok(Response::new(BluetoothStatus { enabled: status }))
    }

    async fn enable_bluetooth(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<EmptyResponse>, Status> {
        let controller = match BluetoothControl::new().await {
            Ok(controller) => controller,
            Err(e) => {
                return Err(Status::from_error(e.into()));
            }
        };

        match controller.enable_bluetooth().await {
            Ok(_) => {}
            Err(e) => {
                return Err(Status::from_error(e.into()));
            }
        };

        Ok(Response::new(EmptyResponse {}))
    }

    async fn disable_bluetooth(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<EmptyResponse>, Status> {
        let controller = match BluetoothControl::new().await {
            Ok(controller) => controller,
            Err(e) => {
                return Err(Status::from_error(e.into()));
            }
        };

        match controller.disable_bluetooth().await {
            Ok(_) => {}
            Err(e) => {
                return Err(Status::from_error(e.into()));
            }
        };

        Ok(Response::new(EmptyResponse {}))
    }
}
