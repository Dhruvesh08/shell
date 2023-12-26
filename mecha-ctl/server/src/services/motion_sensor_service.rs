use tonic::{Request, Response, Status};

pub use mecha_motion_sensor_ctl::MotionSensorControl;

#[derive(Default)]
pub struct MotionSensorManager {
    pub motion_sensor: MotionSensorControl,
}

#[allow(non_snake_case)]
pub mod motionsensor {
    tonic::include_proto!("motionsensor");
}

pub use motionsensor::{
    motion_sensor_control_service_server::{
        MotionSensorControlService, MotionSensorControlServiceServer,
    },
    DetectEventResponse, Empty, ReadValueResponse,
};

#[tonic::async_trait]
impl MotionSensorControlService for MotionSensorManager {
    async fn read_value(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<ReadValueResponse>, Status> {
        match self.motion_sensor.read_motion_sensor_value() {
            Ok((x_value, y_value, z_value)) => {
                // Construct a successful response with the motion sensor values.
                Ok(Response::new(ReadValueResponse {
                    x_value,
                    y_value,
                    z_value,
                }))
            }
            Err(err) => Err(Status::from_error(err.into())),
        }
    }

    async fn detect_motion(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<DetectEventResponse>, Status> {
        match self.motion_sensor.detect_motion_sensor_event() {
            Ok(is_motion_detected) => Ok(Response::new(DetectEventResponse { is_motion_detected })),
            Err(err) => Err(Status::from_error(err.into())),
        }
    }
}
