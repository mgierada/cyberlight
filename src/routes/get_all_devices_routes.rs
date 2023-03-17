use rocket::serde::json::Json;

use crate::services::govee_api_service::{get_all_devices, ApiResponseAllDevices};
use crate::{GOVEE_API_KEY, GOVEE_ROOT_URL};

#[get("/devices")]
pub async fn get_all_devices_handler() -> Json<serde_json::Value> {
    let all_devices_response: ApiResponseAllDevices = get_all_devices(&GOVEE_ROOT_URL, &GOVEE_API_KEY).await;
    let all_devices = all_devices_response.data.unwrap().devices;
    let trim_response = all_devices.iter().map(|device| device.deviceName.clone()).collect::<Vec<String>>();



     

    Json(serde_json::json!({ "devices": trim_response}))
}
