use rocket::serde::json::Json;
use serde::Serialize;

use crate::services::govee_api_service::{
    get_all_devices, get_device_status, ApiResponseAllDevices, ApiResponseDeviceStatus,
};
use crate::{GOVEE_API_KEY, GOVEE_ROOT_URL};

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
struct GoveeDevice {
    deviceName: String,
    device: String,
    model: String,
    controllable: bool,
    retrievable: bool,
}

#[get("/devices")]
pub async fn get_all_devices_handler() -> Json<serde_json::Value> {
    let all_devices_response: ApiResponseAllDevices =
        get_all_devices(&GOVEE_ROOT_URL, &GOVEE_API_KEY).await;
    let all_devices = all_devices_response.data.unwrap().devices;
    let trim_response = all_devices
        .iter()
        .map(|device| GoveeDevice {
            deviceName: device.deviceName.clone(),
            device: device.device.clone(),
            model: device.model.clone(),
            controllable: device.controllable,
            retrievable: device.retrievable,
        })
        .collect::<Vec<GoveeDevice>>();

    Json(serde_json::json!({ "devices": trim_response }))
}

#[get("/status")]
pub async fn get_status_for_all_devices() -> Json<serde_json::Value> {
    let get_response: ApiResponseDeviceStatus =
        get_device_status(&GOVEE_ROOT_URL, &GOVEE_API_KEY, "DF:D7:C1:30:39:39:06:43", "H6076").await;
    Json(serde_json::json!({ "status": get_response }))
}
