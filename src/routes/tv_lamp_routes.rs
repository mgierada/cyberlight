use crate::implementations::access_token::Token;
use crate::services::light_setup_service::tv_light_setup;
use crate::{GOVEE_API_KEY, GOVEE_ROOT_URL};
use govee_api::api::api::control_device;
use rocket::serde::json::Json;

#[get("/on")]
pub async fn tv_on_handler(_token: Token) -> Json<serde_json::Value> {
    let payload = tv_light_setup("on");
    control_device(&GOVEE_ROOT_URL, &GOVEE_API_KEY, payload).await;
    Json(serde_json::json!({"device": "tv_light", "status": "on"}))
}

#[get("/off")]
pub async fn tv_off_handler(_token: Token) -> Json<serde_json::Value> {
    let payload = tv_light_setup("off");
    control_device(&GOVEE_ROOT_URL, &GOVEE_API_KEY, payload).await;
    Json(serde_json::json!({"device": "tv_light", "status": "off"}))
}
