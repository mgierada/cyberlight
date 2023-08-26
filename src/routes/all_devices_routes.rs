use crate::wrappers::all_devices_wrapper::{
    wrap_device_status, wrap_devices, wrap_model_and_devices, DeviceStatus,
};
use crate::GOVEE_API_KEY;
use govee_api::{structs::govee::GoveeDevice, GoveeClient};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;

async fn get_devices_handler(
    govee_client: GoveeClient,
) -> Result<Vec<GoveeDevice>, Custom<Json<serde_json::Value>>> {
    match govee_client.get_devices().await {
        Ok(response) => {
            let raw_devices = response.data.unwrap().devices;
            Ok(raw_devices)
        }
        Err(err) => {
            println!("Error: {}", err);
            let error_json = serde_json::json!({ "error": "Failed to fetch devices" });
            Err(Custom(Status::BadRequest, Json(error_json)))
        }
    }
}

#[get("/devices")]
pub async fn get_all_devices_handler(
) -> Result<Json<serde_json::Value>, Custom<Json<serde_json::Value>>> {
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    match get_devices_handler(govee_client).await {
        Ok(raw_devices) => {
            let wrapped_devices = wrap_devices(raw_devices);
            Ok(Json(serde_json::json!({ "devices": wrapped_devices })))
        }
        Err(err) => {
            Err(err) // Just propagate the error without wrapping it again
        }
    }
}

#[get("/status")]
pub async fn get_status_for_all_devices(
) -> Result<Json<serde_json::Value>, Custom<Json<serde_json::Value>>> {
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    match get_devices_handler(govee_client).await {
        Ok(raw_devices) => {
            let wrapped_models_and_devices = wrap_model_and_devices(raw_devices);
            let mut response_status: Vec<DeviceStatus> = Vec::new();
            for model_and_device in wrapped_models_and_devices {
                let device = model_and_device.device;
                let model = model_and_device.model;
                let govee_client = GoveeClient::new(&GOVEE_API_KEY);
                match govee_client.get_device_state(&device, &model).await {
                    Ok(response) => {
                        let raw_devices_status = response.data.unwrap();
                        let response = wrap_device_status(raw_devices_status);
                        response_status.push(response);
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                        let error_json = serde_json::json!({ "error": "Failed to fetch devices" });
                        return Err(Custom(Status::BadRequest, Json(error_json)));
                    }
                }
            }
            Ok(Json(serde_json::json!({ "status": response_status })))
        }
        Err(err) => Err(err),
    }
}

#[get("/status/<device>/<model>")]
pub async fn get_status_for_device(device: String, model: String) -> Json<serde_json::Value> {
    let govee_client = GoveeClient::new(&GOVEE_API_KEY);
    match govee_client.get_device_state(&device, &model).await {
        Ok(response) => {
            let raw_devices_status = response.data.unwrap();
            let response = wrap_device_status(raw_devices_status);
            Json(serde_json::json!({ "status": response }))
        }
        Err(err) => {
            println!("Error: {}", err);
            let error_json = serde_json::json!({ "error": "Failed to fetch devices" });
            Json(error_json)
        }
    }
}
