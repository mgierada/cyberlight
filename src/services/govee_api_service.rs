use crate::PayloadBody;
use reqwest::Client;
use serde_json::json;

pub async fn sent_put_request(
    govee_api_url: &str,
    govee_api_key: &str,
    payload: PayloadBody,
) -> () {
    let client = Client::new();
    let payload_json = json!(payload);
    let _response = client
        .put(govee_api_url)
        .header("Govee-API-Key", govee_api_key)
        .json(&payload_json)
        .send()
        .await
        .unwrap();
}
