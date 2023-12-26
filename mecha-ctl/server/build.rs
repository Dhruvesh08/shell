fn main() -> Result<(), Box<dyn std::error::Error>> {
    let network_manager = "./proto/network_manager.proto";
    let display_manager = "./proto/display_manager.proto";
    let motion_sensor_manager = "./proto/motion_sensor_manager.proto";
    let led_manager = "./proto/led_manager.proto";
    let device_info = "./proto/device_info.proto";
    let device_metrics = "./proto/metrics_manager.proto";
    let cpu_governor_ctrl = "./proto/cpu_governor_ctrl.proto";
    let trustzone_ctrl = "./proto/trustzone_ctrl.proto";
    let battery_ctrl = "./proto/battery_ctrl.proto";
    let bluetooth_manager = "./proto/bluetooth_manager.proto";

    tonic_build::configure().build_server(true).compile(
        &[
            network_manager,
            display_manager,
            motion_sensor_manager,
            led_manager,
            device_info,
            device_metrics,
            cpu_governor_ctrl,
            trustzone_ctrl,
            battery_ctrl,
            bluetooth_manager,
        ],
        &[
            "./proto/network_manager",
            "./proto/display_manager",
            "./proto/motion_sensor_manager",
            "./proto/led_manager",
            "./proto/device_info",
            "./proto/metrics_manager",
            "./proto/cpu_governor_ctrl",
            "./proto/trustzone_ctrl",
            "./proto/battery_ctrl",
            "./proto/bluetooth_manager",
        ],
    )?;
    Ok(())
}
