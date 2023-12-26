use tonic::{Request, Response, Status};

pub use mecha_led_ctl::{LedColor, LedControl, LedctlError, LedctlErrorCodes};

#[allow(non_snake_case)]
pub mod ledmanager {
    tonic::include_proto!("led_ctrl");
}

pub use ledmanager::{
    ledctl_service_server::{LedctlService, LedctlServiceServer},
    Empty, LedColor as LedColorProto,
};

pub struct LedctlManager {
    pub led_ctl: LedControl,
}

#[tonic::async_trait]
impl LedctlService for LedctlManager {
    async fn set_led(&self, request: Request<LedColorProto>) -> Result<Response<Empty>, Status> {
        let colors = request.into_inner();

        // Extract the red, green, and blue values
        let red = colors.red;
        let green = colors.green;
        let blue = colors.blue;

        // Convert the boolean values to u8
        let red_value = if red { 1 } else { 0 };
        let green_value = if green { 1 } else { 0 };
        let blue_value = if blue { 1 } else { 0 };

        match self
            .led_ctl
            .set_led(red_value as u8, green_value as u8, blue_value as u8)
        {
            Ok(_) => Ok(Response::new(Empty {})),
            Err(err) => Err(Status::from_error(err.into())),
        }
    }

    async fn clear_led(&self, request: Request<LedColorProto>) -> Result<Response<Empty>, Status> {
        let colors = request.into_inner();

        // Extract the red, green, and blue values
        let red = colors.red;
        let green = colors.green;
        let blue = colors.blue;

        // Convert the boolean values to u8
        let red_value = if red { 1 } else { 0 };
        let green_value = if green { 1 } else { 0 };
        let blue_value = if blue { 1 } else { 0 };

        match self
            .led_ctl
            .set_led(red_value as u8, green_value as u8, blue_value as u8)
        {
            Ok(_) => Ok(Response::new(Empty {})),
            Err(err) => Err(Status::from_error(err.into())),
        }
    }
}
