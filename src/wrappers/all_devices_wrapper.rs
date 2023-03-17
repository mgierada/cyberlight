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

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct GoveeModelAndDevice {
    deviceName: String,
    pub device: String,
    pub model: String,
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

pub fn wrap_model_and_devices(devices: Vec<Device>) -> Vec<GoveeModelAndDevice> {
    devices
        .iter()
        .map(|device| GoveeModelAndDevice {
            deviceName: device.deviceName.clone(),
            device: device.device.clone(),
            model: device.model.clone(),
        })
        .collect::<Vec<GoveeModelAndDevice>>()
}
