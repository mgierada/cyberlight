use rocket::serde::json::Json;

use crate::services::govee_api_service::sent_put_request;
use crate::{get_govee_api_key, office_light_setup};

#[get("/on")]
pub async fn office_on_handler() -> Json<serde_json::Value> {
    let govee_api_key = get_govee_api_key();
    let govee_api_url = "https://developer-api.govee.com/v1/devices/control";
    let payload = office_light_setup("on");
    sent_put_request(govee_api_url, &govee_api_key, payload).await;
    Json(serde_json::json!({"device": "office_light", "status": "on"}))
}

#[get("/off")]
pub async fn office_off_handler() -> Json<serde_json::Value> {
    let govee_api_key = get_govee_api_key();
    let govee_api_url = "https://developer-api.govee.com/v1/devices/control";
    let payload = office_light_setup("off");
    sent_put_request(govee_api_url, &govee_api_key, payload).await;
    Json(serde_json::json!({"device": "office_light", "status": "off"}))
}
