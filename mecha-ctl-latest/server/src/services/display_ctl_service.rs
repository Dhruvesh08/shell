use anyhow::Result;
use tonic::{Request, Response, Status};

pub use mecha_display_ctl::DisplayControl;

pub struct Display {
    pub display_ctrl: DisplayControl,
}

#[allow(non_snake_case)]
pub mod displaymanager {
    tonic::include_proto!("displaymanager");
}

pub use displaymanager::{
    display_ctrl_service_server::{DisplayCtrlService, DisplayCtrlServiceServer},
    GetBrightnessRequest, GetBrightnessResponse, SetBrightnessRequest, SetBrightnessResponse,
};

#[tonic::async_trait]
impl DisplayCtrlService for Display {
    async fn set_brightness(
        &self,
        request: Request<SetBrightnessRequest>,
    ) -> Result<Response<SetBrightnessResponse>, Status> {
        let brightness = request.into_inner().brightness;

        match self.display_ctrl.set_display_brightness(brightness as u8) {
            Ok(_) => Ok(Response::new(SetBrightnessResponse {})), // Return a successful response.
            Err(err) => {
                // Convert the error into a gRPC status and return it.
                Err(Status::from_error(err.into()))
            }
        }
    }

    async fn get_brightness(
        &self,
        _request: Request<GetBrightnessRequest>,
    ) -> Result<Response<GetBrightnessResponse>, Status> {
        match self.display_ctrl.get_display_brightness() {
            Ok(brightness) => {
                // Construct a successful response with the brightness value.
                Ok(Response::new(GetBrightnessResponse {
                    brightness: brightness.into(),
                }))
            }
            Err(err) => Err(Status::from_error(err.into())),
        }
    }
}
