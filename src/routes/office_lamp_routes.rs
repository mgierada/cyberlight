use rocket::serde::json::Json;

use crate::services::govee_api_service::sent_put_request;
use crate::services::light_setup_service::office_light_setup;
use crate::{get_goove_root_url, GOVEE_API_KEY};

#[get("/on")]
pub async fn office_on_handler() -> Json<serde_json::Value> {
    let govee_root_url = get_goove_root_url();
    let payload = office_light_setup("on");
    sent_put_request(&govee_root_url, &GOVEE_API_KEY, payload).await;
    Json(serde_json::json!({"device": "office_light", "status": "on"}))
}

#[get("/off")]
pub async fn office_off_handler() -> Json<serde_json::Value> {
    let govee_root_url = get_goove_root_url();
    let payload = office_light_setup("off");
    sent_put_request(&govee_root_url, &GOVEE_API_KEY, payload).await;
    Json(serde_json::json!({"device": "office_light", "status": "off"}))
}
