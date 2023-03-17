use serde::Serialize;

use crate::services::govee_api_service::Device;

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct GoveeDevice {
    deviceName: String,
    device: String,
    model: String,
    controllable: bool,
    retrievable: bool,
}

pub fn wrap_devices(devices: Vec<Device>) -> Vec<GoveeDevice> {
    devices
        .iter()
        .map(|device| GoveeDevice {
            deviceName: device.deviceName.clone(),
            device: device.device.clone(),
            model: device.model.clone(),
            controllable: device.controllable,
            retrievable: device.retrievable,
        })
        .collect::<Vec<GoveeDevice>>()
}
