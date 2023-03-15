use rocket::serde::json::Json;

use crate::services::govee_api_service::sent_put_request;
use crate::services::light_setup_service::tv_light_setup;
use crate::{GOVEE_API_KEY, GOVEE_ROOT_URL};

#[get("/on")]
pub async fn tv_on_handler() -> Json<serde_json::Value> {
    let payload = tv_light_setup("on");
    sent_put_request(&GOVEE_ROOT_URL, &GOVEE_API_KEY, payload).await;
    Json(serde_json::json!({"device": "tv_light", "status": "on"}))
}

#[get("/off")]
pub async fn tv_off_handler() -> Json<serde_json::Value> {
    let payload = tv_light_setup("off");
    sent_put_request(&GOVEE_ROOT_URL, &GOVEE_API_KEY, payload).await;
    Json(serde_json::json!({"device": "tv_light", "status": "off"}))
}
