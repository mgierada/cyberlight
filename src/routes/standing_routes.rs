use crate::constants::enums::OfficeDevices;
use crate::implementations::access_token::Token;
use crate::services::light_setup_service::office_light_setup;
use crate::GOVEE_API_KEY;
use govee_api::GoveeClient;
use rocket::serde::json::Json;

#[get("/right/on")]
pub async fn standing_right_on_handler(_token: Token) -> Json<serde_json::Value> {
    let standing_right_led = OfficeDevices::standing_right_led();
    let payload = office_light_setup(&standing_right_led, "on");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let result = govee_client.control_device(payload).await;
    if let Err(err) = result {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"device": "standing_right_led", "status": "on"}))
}

#[get("/right/off")]
pub async fn standing_right_off_handler(_token: Token) -> Json<serde_json::Value> {
    let standing_right_led = OfficeDevices::standing_right_led();
    let payload = office_light_setup(&standing_right_led, "off");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let result = govee_client.control_device(payload).await;
    if let Err(err) = result {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"device": "standing_right_led", "status": "off"}))
}

#[get("/left/on")]
pub async fn standing_left_on_handler(_token: Token) -> Json<serde_json::Value> {
    let standing_left_led = OfficeDevices::standing_left_led();
    let payload = office_light_setup(&standing_left_led, "on");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let result = govee_client.control_device(payload).await;
    if let Err(err) = result {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"device": "standing_left_led", "status": "on"}))
}

#[get("/left/off")]
pub async fn standing_left_off_handler(_token: Token) -> Json<serde_json::Value> {
    let standing_left_led = OfficeDevices::standing_left_led();
    let payload = office_light_setup(&standing_left_led, "off");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let result = govee_client.control_device(payload).await;
    if let Err(err) = result {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"device": "standing_left_led", "status": "off"}))
}
