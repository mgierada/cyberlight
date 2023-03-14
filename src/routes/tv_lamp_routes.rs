use rocket::serde::json::Json;

use crate::{get_govee_api_key, tv_light_setup};
use crate::services::govee_api_service::sent_put_request;

#[get("/on")]
pub async fn tv_on_handler() -> Json<serde_json::Value> {
    let govee_api_key = get_govee_api_key();
    let govee_api_url = "https://developer-api.govee.com/v1/devices/control";
    let payload = tv_light_setup("on");
    sent_put_request(govee_api_url, &govee_api_key, payload).await
}

#[get("/off")]
pub async fn tv_off_handler() -> Json<serde_json::Value> {
    let govee_api_key = get_govee_api_key();
    let govee_api_url = "https://developer-api.govee.com/v1/devices/control";
    let payload = tv_light_setup("off");
    sent_put_request(govee_api_url, &govee_api_key, payload).await
}

