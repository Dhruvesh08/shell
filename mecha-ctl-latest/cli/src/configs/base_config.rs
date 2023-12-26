use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseConfig {
    pub server: GrpcConfig,
    pub interfaces: Interfaces,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrpcConfig {
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Interfaces {
    pub display: Display,
    pub motion_sensor: Gyroscope,
    pub led: Led,
    pub battery: Battery,
}
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Display {
    pub device: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Gyroscope {
    pub x_axis: String,
    pub y_axis: String,
    pub z_axis: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Led {
    pub red_led: String,
    pub green_led: String,
    pub blue_led: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Battery {
    pub device: String,
    pub current: String,
}
