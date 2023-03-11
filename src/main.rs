#[macro_use]
extern crate rocket;
use dotenv::dotenv;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize)]
struct MyRequestBody {
    // Define the fields of the request body here
    device: String,
    model: String,
    command: Command,
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

#[get("/on")]
async fn on_handler() -> Json<serde_json::Value> {
    // load env vars
    dotenv().ok();

    let goove_api_key = std::env::var("GOVEE_API_KEY")
        .expect("GOVEE_API_KEY must be set.")
        .to_string();
    let goove_api_device = std::env::var("GOVEE_DEVICE_ID")
        .expect("GOVEE_DEVICE_ID must be set")
        .to_string();
    let goove_model = std::env::var("GOVEE_MODEL")
        .expect("GOVEE_MODEL must be set")
        .to_string();
    let command = Command {
        name: "turn".to_string(),
        value: "on".to_string(),
    };
    let govee_api_url = "https://developer-api.govee.com/v1/devices/control";

    let payload = json!({
        "device": goove_api_device,
        "model": goove_model,
        "cmd": command,
    });

    let client = reqwest::Client::new();
    let response = client
        .put(govee_api_url)
        .header("Govee-API-Key", goove_api_key)
        .json(&payload)
        .send()
        .await
        .unwrap();
    println!("response: {:?}", response);
    println!("response: {:?}", response.text().await.unwrap());
    Json(serde_json::json!({"status": "done"}))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![on_handler])
}
