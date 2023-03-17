use rocket::serde::json::Json;

use crate::services::govee_api_service::{
    get_all_devices, get_device_status, ApiResponseAllDevices, ApiResponseDeviceStatus,
};
use crate::wrappers::all_devices_wrapper::wrap_devices;
use crate::{GOVEE_API_KEY, GOVEE_ROOT_URL};

#[get("/devices")]
pub async fn get_all_devices_handler() -> Json<serde_json::Value> {
    let response: ApiResponseAllDevices = get_all_devices(&GOVEE_ROOT_URL, &GOVEE_API_KEY).await;
    let raw_devices = response.data.unwrap().devices;
    let wrapped_devices = wrap_devices(raw_devices);
    Json(serde_json::json!({ "devices": wrapped_devices }))
}

#[get("/status")]
pub async fn get_status_for_all_devices() -> Json<serde_json::Value> {
    let get_response: ApiResponseDeviceStatus =
        get_device_status(&GOVEE_ROOT_URL, &GOVEE_API_KEY, "aaa", "H6076").await;
    Json(serde_json::json!({ "status": get_response }))
}
