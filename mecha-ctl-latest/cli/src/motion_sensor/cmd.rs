use anyhow::Result;
use clap::{Args, Subcommand};

pub use mecha_motion_sensor_ctl::MotionSensorControl;

use crate::{
    configs::BaseConfig,
    output_message::{Message, StdOut, MOTION},
};

#[derive(Debug, Args)]
pub struct MotionSensor {
    #[command(subcommand)]
    command: MotionSensorCommands,
}

#[derive(Debug, Subcommand)]
enum MotionSensorCommands {
    #[command(about = "Get motion sensor value")]
    Value,
    #[command(about = "Get motion sensor event")]
    Event,
}

impl MotionSensor {
    pub async fn execute(&self, config: &BaseConfig) -> Result<()> {
        let x_axis_path = config.interfaces.motion_sensor.x_axis.clone();
        let y_axis_path = config.interfaces.motion_sensor.y_axis.clone();
        let z_axis_path = config.interfaces.motion_sensor.z_axis.clone();

        let motion_sensor = MotionSensorControl::new(&x_axis_path, &y_axis_path, &z_axis_path);

        match &self.command {
            MotionSensorCommands::Value => match motion_sensor.read_motion_sensor_value() {
                Ok((x, y, z)) => {
                    println!();
                    StdOut::info(
                        &format!("Motion sensor value: x: {}, y: {}, z: {}", x, y, z),
                        Some(MOTION),
                    )
                }
                Err(err) => return Err(err.into()),
            },
            MotionSensorCommands::Event => match motion_sensor.detect_motion_sensor_event() {
                Ok(event) => StdOut::info(
                    &format!("Motion sensor event: event: {}", event),
                    Some(MOTION),
                ),
                Err(err) => return Err(err.into()),
            },
        }
        Ok(())
    }
}
