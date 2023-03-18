use serde::Serialize;

use crate::services::govee_api_service::{DataDeviceStatus, Device, DeviceProperty};

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

#[derive(Debug, Serialize)]
pub struct GoveeDeviceStatus {
    device: String,
    model: String,
    properties: Vec<GoveeDeviceProperty>,
}

#[derive(Debug, Serialize)]
enum GoveeDeviceProperty {
    #[serde(rename = "online")]
    Online(bool),
    #[serde(rename = "powerState")]
    PowerState(String),
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

pub fn wrap_device_status(device: DataDeviceStatus) -> GoveeDeviceStatus {
    GoveeDeviceStatus {
        device: device.device.clone(),
        model: device.model.clone(),
        properties: device
            .properties
            .iter()
            .map(|property| match property {
                DeviceProperty::Online(online) => GoveeDeviceProperty::Online(*online),
                DeviceProperty::PowerState(power_state) => {
                    GoveeDeviceProperty::PowerState(power_state.clone())
                }
                _ => panic!("Unexpected property type"),
            })
            .collect::<Vec<GoveeDeviceProperty>>(),
    }
}
