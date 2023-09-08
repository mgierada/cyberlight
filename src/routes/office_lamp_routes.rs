use govee_api::GoveeClient;
use rocket::serde::json::Json;

use crate::constants::enums::OfficeDevices;
use crate::implementations::access_token::Token;
use crate::services::light_setup_service::office_light_setup;
use crate::GOVEE_API_KEY;

#[get("/on")]
pub async fn office_on_handler(_token: Token) -> Json<serde_json::Value> {
    let payload = office_light_setup("on");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let result = govee_client.control_device(payload).await;
    if let Err(err) = result {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"device": "office_light", "status": "on"}))
}

#[get("/off")]
pub async fn office_off_handler(_token: Token) -> Json<serde_json::Value> {
    let payload = office_light_setup("off");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let result = govee_client.control_device(payload).await;
    if let Err(err) = result {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"device": "office_light", "status": "off"}))
}

#[get("/table/on")]
pub async fn office_table_on_handler(_token: Token) -> Json<serde_json::Value> {
    let payload = office_light_setup(OfficeDevices::TableLED(_), "on");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let result = govee_client.control_device(payload).await;
    if let Err(err) = result {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"device": "office_light", "status": "off"}))
}
