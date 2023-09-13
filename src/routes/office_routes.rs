use govee_api::GoveeClient;
use rocket::serde::json::Json;

use crate::constants::enums::OfficeDevices;
use crate::implementations::access_token::Token;
use crate::services::light_setup_service::office_light_setup;
use crate::GOVEE_API_KEY;

#[get("/on")]
pub async fn office_on_handler(_token: Token) -> Json<serde_json::Value> {
    // TODO Refactor this ugly implementation when bulk control is available
    let corner_led = OfficeDevices::corner_led();
    let table_led = OfficeDevices::table_led();
    let window_led = OfficeDevices::window_led();
    let board_led = OfficeDevices::board_led();
    let payload_corner = office_light_setup(&corner_led, "on");
    let payload_table = office_light_setup(&table_led, "on");
    let payload_window = office_light_setup(&window_led, "on");
    let payload_board = office_light_setup(&board_led, "on");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let resuts = govee_client.bulk_control_devices(vec![payload_corner, payload_table, payload_window, payload_board]).await;
    if let Err(err) = resuts {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"device": "all", "status": "on"}))
}

#[get("/off")]
pub async fn office_off_handler(_token: Token) -> Json<serde_json::Value> {
    // TODO Refactor this ugly implementation when bulk control is available
    let corner_led = OfficeDevices::corner_led();
    let table_led = OfficeDevices::table_led();
    let window_led = OfficeDevices::window_led();
    let board_led = OfficeDevices::board_led();
    let payload_corner = office_light_setup(&corner_led, "off");
    let payload_table = office_light_setup(&table_led, "off");
    let payload_window = office_light_setup(&window_led, "off");
    let payload_board = office_light_setup(&board_led, "off");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let resuts = govee_client.bulk_control_devices(vec![payload_corner, payload_table, payload_window, payload_board]).await;
    if let Err(err) = resuts {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"device": "all", "status": "on"}))
}

#[get("/board/on")]
pub async fn office_board_on_handler(_token: Token) -> Json<serde_json::Value> {
    let board_led = OfficeDevices::board_led();
    let payload = office_light_setup(&board_led, "on");
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
    let payload = office_light_setup(&board_led, "off");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let result = govee_client.control_device(payload).await;
    if let Err(err) = result {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"device": "board_led", "status": "off"}))
}

#[get("/corner/on")]
pub async fn office_corner_on_handler(_token: Token) -> Json<serde_json::Value> {
    let corner_led = OfficeDevices::corner_led();
    let payload = office_light_setup(&corner_led, "on");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let result = govee_client.control_device(payload).await;
    if let Err(err) = result {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"device": "corner_led", "status": "on"}))
}

#[get("/corner/off")]
pub async fn office_corner_off_handler(_token: Token) -> Json<serde_json::Value> {
    let corner_led = OfficeDevices::corner_led();
    let payload = office_light_setup(&corner_led, "off");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let result = govee_client.control_device(payload).await;
    if let Err(err) = result {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"device": "corner_led", "status": "off"}))
}

#[get("/table/on")]
pub async fn office_table_on_handler(_token: Token) -> Json<serde_json::Value> {
    let table_led = OfficeDevices::table_led();
    let payload = office_light_setup(&table_led, "on");
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
    let payload = office_light_setup(&table_led, "off");
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
    let payload = office_light_setup(&window_led, "on");
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
    let payload = office_light_setup(&window_led, "off");
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let result = govee_client.control_device(payload).await;
    if let Err(err) = result {
        panic!("Error occurred: {:?}", err);
    }
    Json(serde_json::json!({"device": "window_led", "status": "off"}))
}
