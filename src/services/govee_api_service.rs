use reqwest::Client;
use rocket::serde::json::Json;
use serde_json::json;
use crate::PayloadBody;

pub async fn sent_put_request(
    govee_api_url: &str,
    govee_api_key: &str,
    payload: PayloadBody,
) -> Json<serde_json::Value> {
    let client = Client::new();
    let payload_json = json!(payload);
    let _response = client
        .put(govee_api_url)
        .header("Govee-API-Key", govee_api_key)
        .json(&payload_json)
        .send()
        .await
        .unwrap();
    Json(serde_json::json!({"status": "done"}))
}
