use rocket::serde::json::Json;

use crate::services::govee_api_service::sent_put_request;
use crate::{get_govee_api_key, tv_light_setup, get_goove_root_url};

#[get("/on")]
pub async fn tv_on_handler() -> Json<serde_json::Value> {
    let govee_api_key = get_govee_api_key();
    let govee_root_url = get_goove_root_url();
    let payload = tv_light_setup("on");
    sent_put_request(&govee_root_url, &govee_api_key, payload).await;
    Json(serde_json::json!({"device": "tv_light", "status": "on"}))
}

#[get("/off")]
pub async fn tv_off_handler() -> Json<serde_json::Value> {
    let govee_api_key = get_govee_api_key();
    let govee_root_url = get_goove_root_url();
    let payload = tv_light_setup("off");
    sent_put_request(&govee_root_url, &govee_api_key, payload).await;
    Json(serde_json::json!({"device": "tv_light", "status": "off"}))
}
