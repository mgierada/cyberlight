use crate::wrappers::all_devices_wrapper::{
    wrap_device_status, wrap_devices, wrap_model_and_devices, DeviceStatus,
};
use crate::GOVEE_API_KEY;
use govee_api::structs::govee::{ApiResponseGoveeDeviceState, ApiResponseGoveeDevices};
use govee_api::GoveeClient;
use rocket::serde::json::Json;

#[get("/devices")]
pub async fn get_all_devices_handler() -> Json<serde_json::Value> {
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let response: ApiResponseGoveeDevices = govee_client.get_devices().await;
    let raw_devices = response.data.unwrap().devices;
    let wrapped_devices = wrap_devices(raw_devices);
    Json(serde_json::json!({ "devices": wrapped_devices }))
}

#[get("/status")]
pub async fn get_status_for_all_devices() -> Json<serde_json::Value> {
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let response: ApiResponseGoveeDevices = govee_client.get_devices().await;
    let raw_devices = response.data.unwrap().devices;
    let wrapped_models_and_devices = wrap_model_and_devices(raw_devices);

    let mut response_status: Vec<DeviceStatus> = Vec::new();
    for model_and_device in wrapped_models_and_devices {
        let device = model_and_device.device;
        let model = model_and_device.model;
        let govee_client = GoveeClient::new(&GOVEE_API_KEY);
        let raw_response: ApiResponseGoveeDeviceState =
            govee_client.get_device_state(&device, &model).await;
        let raw_devices_status = raw_response.data.unwrap();
        let response = wrap_device_status(raw_devices_status);
        response_status.push(response);
    }
    Json(serde_json::json!({ "status": response_status }))
}

#[get("/status/<device>/<model>")]
pub async fn get_status_for_device(device: String, model: String) -> Json<serde_json::Value> {
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    let raw_response: ApiResponseGoveeDeviceState =
        govee_client.get_device_state(&device, &model).await;
    let raw_devices_status = raw_response.data.unwrap();
    Json(serde_json::json!({ "status": raw_devices_status }))
}
