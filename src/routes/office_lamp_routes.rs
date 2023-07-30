use govee_api::api::api::control_device;
use rocket::serde::json::Json;

use crate::implementations::access_token::Token;
use crate::services::light_setup_service::office_light_setup;
use crate::{GOVEE_API_KEY, GOVEE_ROOT_URL};

#[get("/on")]
pub async fn office_on_handler(_token: Token) -> Json<serde_json::Value> {
    let payload = office_light_setup("on");
    control_device(&GOVEE_ROOT_URL, &GOVEE_API_KEY, payload).await;
    Json(serde_json::json!({"device": "office_light", "status": "on"}))
}

#[get("/off")]
pub async fn office_off_handler(_token: Token) -> Json<serde_json::Value> {
    let payload = office_light_setup("off");
    control_device(&GOVEE_ROOT_URL, &GOVEE_API_KEY, payload).await;
    Json(serde_json::json!({"device": "office_light", "status": "off"}))
}
