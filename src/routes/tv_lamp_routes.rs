use crate::implementations::access_token::Token;
use crate::services::light_setup_service::tv_light_setup;
use crate::GOVEE_API_KEY;
use govee_api::GoveeClient;
use rocket::serde::json::Json;

#[get("/on")]
pub async fn tv_on_handler(_token: Token) -> Json<serde_json::Value> {
    let payload = tv_light_setup("on");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    govee_client.control_device(payload).await;
    Json(serde_json::json!({"device": "tv_light", "status": "on"}))
}

#[get("/off")]
pub async fn tv_off_handler(_token: Token) -> Json<serde_json::Value> {
    let payload = tv_light_setup("off");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    govee_client.control_device(payload).await;
    Json(serde_json::json!({"device": "tv_light", "status": "off"}))
}
