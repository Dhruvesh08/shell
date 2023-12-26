use anyhow::Result;
pub use mecha_battery_ctl::{Battery, PowerSupplyInfo};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct BatteryControl {
    pub power_supply: Battery,
}

pub mod power_supply {
    tonic::include_proto!("battery");
}

pub use power_supply::{
    power_supply_service_server::{PowerSupplyService, PowerSupplyServiceServer},
    Empty, GetCurrentResponse, GetDeviceResponse, GetPowerSupplyInfoResponse, SetDeviceRequest,
};

#[tonic::async_trait]
impl PowerSupplyService for BatteryControl {
    async fn get_power_supply_info(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<GetPowerSupplyInfoResponse>, Status> {
        let power_supply_info = match self.power_supply.info() {
            Ok(info) => info,
            Err(err) => return Err(Status::from_error(err.into())),
        };

        let respose = GetPowerSupplyInfoResponse {
            name: power_supply_info.name,
            r#type: power_supply_info.r#type,
            status: power_supply_info.status,
            present: power_supply_info.present,
            voltage_now: power_supply_info.voltage_now,
            current_now: power_supply_info.current_now,
            capacity: power_supply_info.capacity.to_string(),
            capacity_level: power_supply_info.capacity_level,
            temp: power_supply_info.temp,
            technology: power_supply_info.technology,
            charge_full: power_supply_info.charge_full,
            charge_now: power_supply_info.charge_now,
            charge_full_design: power_supply_info.charge_full_design,
            manufacturer: power_supply_info.manufacturer,
        };

        Ok(Response::new(respose))
    }

    async fn set_device(
        &self,
        request: Request<SetDeviceRequest>,
    ) -> Result<Response<Empty>, Status> {
        let _device = request.into_inner().device_path;
        // match self.power_supply.set_device(&device) {
        //     Ok(_) => Ok(Response::new(Empty {})),
        //     Err(err) => Err(Status::from_error(err.into())),
        // }

        Ok(Response::new(Empty {}))
    }

    async fn get_device(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<GetDeviceResponse>, Status> {
        let device = match self.power_supply.get_device() {
            Ok(device) => device,
            Err(err) => return Err(Status::from_error(err.into())),
        };

        let response = GetDeviceResponse {
            device_path: device.to_string(),
        };

        Ok(Response::new(response))
    }

    async fn get_current(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<GetCurrentResponse>, Status> {
        let current = match self.power_supply.get_current() {
            Ok(current) => current,
            Err(err) => return Err(Status::from_error(err.into())),
        };

        let response = GetCurrentResponse {
            current_value: current,
        };

        Ok(Response::new(response))
    }
}
