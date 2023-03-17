use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::light_setup_service::PayloadBody;

// ------------------------
// Structs for the Govee API
// ------------------------

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponseDeviceStatus {
    code: i16,
    message: String,
    data: Option<DataDeviceStatus>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataDeviceStatus {
    device: String,
    model: String,
    properties: DeviceProperties,
}

#[derive(Debug, Deserialize, Serialize)]
enum DeviceProperties {
    StatusOnline(StatusOnline),
    StatusPowerState(StatusPowerState),
    StatusColor(StatusColor),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StatusOnline {
    online: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct StatusPowerState {
    powerState: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StatusColor {
    color: Color,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Color {
    r: i16,
    g: i16,
    b: i16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponseAllDevices {
    code: i16,
    message: String,
    pub data: Option<Data>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub devices: Vec<Device>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Device {
    pub device: String,
    pub model: String,
    pub deviceName: String,
    pub controllable: bool,
    pub retrievable: bool,
    pub supportCmds: Vec<String>,
    pub properties: Properties,
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

pub async fn get_device_status(
    govee_root_url: &str,
    govee_api_key: &str,
    device: &str,
    model: &str,
) -> ApiResponseDeviceStatus {
    let client = Client::new();
    let params = [("device", device), ("model", model)];
    let url = Url::parse_with_params(govee_root_url, &params).unwrap();
    let response = client
        .get(url)
        .header("Govee-API-Key", govee_api_key)
        .send()
        .await
        .unwrap()
        .json::<ApiResponseDeviceStatus>();
    println!("response: {:?}", response);
    let response_json: ApiResponseDeviceStatus = response.await.unwrap();
    response_json
}
