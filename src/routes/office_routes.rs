use govee_api::GoveeClient;
use govee_api::structs::govee::PayloadBody;
use rocket::serde::json::Json;

use crate::constants::enums::OfficeDevices;
use crate::implementations::access_token::Token;
use crate::services::light_setup_service::office_setup;
use crate::GOVEE_API_KEY;

#[get("/on")]
pub async fn office_on_handler(_token: Token) -> Json<serde_json::Value> {
    let devices = [
        OfficeDevices::standing_left_led(),
        OfficeDevices::standing_right_led(),
        OfficeDevices::table_led(),
        OfficeDevices::window_led(),
        OfficeDevices::board_led(),
    ];
    let payloads: Vec<PayloadBody> = devices
        .iter()
        .map(|device| office_setup(device, "on"))
        .collect();

    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    if let Err(err) = govee_client.bulk_control_devices(payloads.clone()).await {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"device": "all", "status": "on"}))
}

#[get("/off")]
pub async fn office_off_handler(_token: Token) -> Json<serde_json::Value> {
    let devices = [
        OfficeDevices::standing_left_led(),
        OfficeDevices::standing_right_led(),
        OfficeDevices::table_led(),
        OfficeDevices::window_led(),
        OfficeDevices::board_led(),
    ];
    let payloads: Vec<PayloadBody> = devices
        .iter()
        .map(|device| office_setup(device, "off"))
        .collect();

    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    if let Err(err) = govee_client.bulk_control_devices(payloads.clone()).await {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"device": "all", "status": "on"}))
}

#[get("/board/on")]
pub async fn office_board_on_handler(_token: Token) -> Json<serde_json::Value> {
    let board_led = OfficeDevices::board_led();
    let payload = office_setup(&board_led, "on");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let result = govee_client.control_device(payload).await;
    if let Err(err) = result {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"device": "board_led", "status": "on"}))
}

#[get("/board/off")]
pub async fn office_board_off_on_handler(_token: Token) -> Json<serde_json::Value> {
    let board_led = OfficeDevices::board_led();
    let payload = office_setup(&board_led, "off");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let result = govee_client.control_device(payload).await;
    if let Err(err) = result {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"device": "board_led", "status": "off"}))
}


#[get("/table/on")]
pub async fn office_table_on_handler(_token: Token) -> Json<serde_json::Value> {
    let table_led = OfficeDevices::table_led();
    let payload = office_setup(&table_led, "on");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let result = govee_client.control_device(payload).await;
    if let Err(err) = result {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"device": "table_led", "status": "on"}))
}

#[get("/table/off")]
pub async fn office_table_off_handler(_token: Token) -> Json<serde_json::Value> {
    let table_led = OfficeDevices::table_led();
    let payload = office_setup(&table_led, "off");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let result = govee_client.control_device(payload).await;
    if let Err(err) = result {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"device": "table_led", "status": "off"}))
}

#[get("/window/on")]
pub async fn office_window_on_handler(_token: Token) -> Json<serde_json::Value> {
    let window_led = OfficeDevices::window_led();
    let payload = office_setup(&window_led, "on");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let result = govee_client.control_device(payload).await;
    if let Err(err) = result {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"device": "window_led", "status": "on"}))
}

#[get("/window/off")]
pub async fn office_window_off_handler(_token: Token) -> Json<serde_json::Value> {
    let window_led = OfficeDevices::window_led();
    let payload = office_setup(&window_led, "off");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let result = govee_client.control_device(payload).await;
    if let Err(err) = result {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"device": "window_led", "status": "off"}))
}


#[get("/humidifier/on")]
pub async fn office_humidifier_on_handler(_token: Token) -> Json<serde_json::Value> {
    let humidifier = OfficeDevices::humidifier();
    let payload = office_setup(&humidifier, "on");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let result = govee_client.control_appliance(payload).await;
    if let Err(err) = result {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"applience": "humidifier", "status": "on"}))
}

#[get("/humidifier/off")]
pub async fn office_humidifier_off_handler(_token: Token) -> Json<serde_json::Value> {
    let humidifier = OfficeDevices::humidifier();
    let payload = office_setup(&humidifier, "off");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let result = govee_client.control_appliance(payload).await;
    if let Err(err) = result {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"applience": "humidifier", "status": "off"}))
}
