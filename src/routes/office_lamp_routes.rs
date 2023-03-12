use rocket::serde::json::Json;

use crate::{get_govee_api_key, office_light_setup, sent_put_request};


#[get("/on")]
pub async fn office_on_handler() -> Json<serde_json::Value> {
    let govee_api_key = get_govee_api_key();
    let govee_api_url = "https://developer-api.govee.com/v1/devices/control";
    let payload = office_light_setup("on");
    sent_put_request(govee_api_url, &govee_api_key, payload).await
}

#[get("/off")]
pub async fn office_off_handler() -> Json<serde_json::Value> {
    let govee_api_key = get_govee_api_key();
    let govee_api_url = "https://developer-api.govee.com/v1/devices/control";
    let payload = office_light_setup("off");
    sent_put_request(govee_api_url, &govee_api_key, payload).await
}
