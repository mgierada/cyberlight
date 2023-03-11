#[macro_use]
extern crate rocket;
use dotenv::dotenv;
use reqwest::Client;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize)]
struct PayloadBody {
    // Define the fields of the request body here
    device: String,
    model: String,
    cmd: Command,
}

#[derive(Serialize, Deserialize, Debug)]
struct Command {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Data {}

#[derive(Debug, Deserialize, Serialize)]
struct ApiResponse {
    code: i32,
    message: String,
    data: Option<Data>,
}
// const GOVEE_API_KEY: &str = match std::env::var("API_KEY") {
//     Ok(val) => val.as_str(),
//     Err(_) => panic!("API_KEY environment variable not set"),
// };
//

fn get_govee_api_key() -> String {
    dotenv().ok();
    std::env::var("GOVEE_API_KEY").expect("GOVEE_API_KEY must be set.")
}

fn tv_light_setup(command: &str) -> PayloadBody {
    let goove_api_device =
        std::env::var("GOVEE_DEVICE_ID_TV_LIGHT").expect("GOVEE_DEVICE_ID must be set");
    let goove_model = std::env::var("GOVEE_MODEL_TV_LIGHT").expect("GOVEE_MODEL must be set");
    let command = Command {
        name: "turn".to_string(),
        value: command.to_string(),
    };
    PayloadBody {
        device: goove_api_device,
        model: goove_model,
        cmd: command,
    }
}

async fn sent_request(
    govee_api_url: &str,
    govee_api_key: &str,
    payload: PayloadBody,
) -> Json<serde_json::Value> {
    let client = Client::new();
    let payload_json = json!(payload);
    println!("payload_json: {:?}", payload_json);
    let response = client
        .put(govee_api_url)
        .header("Govee-API-Key", govee_api_key)
        .json(&payload_json)
        .send()
        .await
        .unwrap();
    println!("response: {:?}", response.text().await.unwrap());
    Json(serde_json::json!({"status": "done"}))
}

#[get("/on")]
async fn tv_on_handler() -> Json<serde_json::Value> {
    let govee_api_key = get_govee_api_key();
    let govee_api_url = "https://developer-api.govee.com/v1/devices/control";
    let payload = tv_light_setup("on");
    sent_request(govee_api_url, &govee_api_key, payload).await
}

#[get("/off")]
async fn tv_off_handler() -> Json<serde_json::Value> {
    let govee_api_key = get_govee_api_key();
    let govee_api_url = "https://developer-api.govee.com/v1/devices/control";
    let payload = tv_light_setup("off");
    sent_request(govee_api_url, &govee_api_key, payload).await
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/tv", routes![tv_on_handler, tv_off_handler])
}
