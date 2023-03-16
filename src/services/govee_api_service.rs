use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::light_setup_service::PayloadBody;

// ------------------------
// Structs for the Govee API
// ------------------------

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponseAllDevices {
    code: i16,
    message: String,
    data: Option<Data>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    devices: Vec<Device>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Device {
    device: String,
    model: String,
    deviceName: String,
    controllable: bool,
    retrievable: bool,
    supportCmds: Vec<String>,
    properties: Properties,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Properties {
    colorTem: ColorTem,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ColorTem {
    range: ColorTemRange,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ColorTemRange {
    min: i16,
    max: i16,
}

// ------------------------
// Methods for the Govee API
// ------------------------

pub async fn sent_put_request(
    govee_root_url: &str,
    govee_api_key: &str,
    payload: PayloadBody,
) -> () {
    let client = Client::new();
    let payload_json = json!(payload);
    let endpoint = format!("{}/v1/devices/control", govee_root_url);
    let _response = client
        .put(endpoint)
        .header("Govee-API-Key", govee_api_key)
        .json(&payload_json)
        .send()
        .await
        .unwrap();
}

pub async fn get_all_devices(govee_root_url: &str, govee_api_key: &str) -> ApiResponseAllDevices {
    let client = Client::new();
    let endpoint = format!("{}/v1/devices", govee_root_url);
    let response = client
        .get(endpoint)
        .header("Govee-API-Key", govee_api_key)
        .send()
        .await
        .unwrap()
        .json::<ApiResponseAllDevices>();
    let response_json: ApiResponseAllDevices = response.await.unwrap();
    response_json
}
