use serde::Serialize;

use crate::services::govee_api_service::{GoveeDataDeviceStatus, GoveeDevice, GoveeDeviceProperty};

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Device {
    deviceName: String,
    device: String,
    model: String,
    controllable: bool,
    retrievable: bool,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct ModelAndDevice {
    deviceName: String,
    pub device: String,
    pub model: String,
}
#[derive(Debug, Serialize)]
pub struct DeviceStatus {
    device: String,
    model: String,
    properties: Vec<GoveeDeviceProperty>,
}

pub fn wrap_devices(devices: Vec<GoveeDevice>) -> Vec<Device> {
    devices
        .iter()
        .map(|device| Device {
            deviceName: device.deviceName.clone(),
            device: device.device.clone(),
            model: device.model.clone(),
            controllable: device.controllable,
            retrievable: device.retrievable,
        })
        .collect::<Vec<Device>>()
}

pub fn wrap_model_and_devices(devices: Vec<GoveeDevice>) -> Vec<ModelAndDevice> {
    devices
        .iter()
        .map(|device| ModelAndDevice {
            deviceName: device.deviceName.clone(),
            device: device.device.clone(),
            model: device.model.clone(),
        })
        .collect::<Vec<ModelAndDevice>>()
}

pub fn wrap_device_status(device: GoveeDataDeviceStatus) -> DeviceStatus {
    let properties = device
        .properties
        .iter()
        .filter_map(|property| match property {
            GoveeDeviceProperty::Online(value) => Some(GoveeDeviceProperty::Online(*value)),
            GoveeDeviceProperty::PowerState(value) => {
                Some(GoveeDeviceProperty::PowerState(value.clone()))
            }
            _ => None,
        })
        .collect::<Vec<GoveeDeviceProperty>>();
    DeviceStatus {
        device: device.device.clone(),
        model: device.model.clone(),
        // properties will return
        properties,
    }
}
