
use rocket::serde::json::Json;

use crate::services::govee_api_service::get_all_devices;
use crate::{GOVEE_API_KEY, GOVEE_ROOT_URL};

#[get("/devices")]
pub async fn get_all_devices_handler() -> Json<serde_json::Value> {
    let all_devices = get_all_devices(&GOVEE_ROOT_URL, &GOVEE_API_KEY);
    Json(serde_json::json!({"all_devices": all_devices}))
}
