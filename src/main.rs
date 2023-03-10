#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use serde_json::{json, Result};
use dotenv::dotenv;
use reqwest::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Command {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Data{}

#[derive(Debug, Deserialize, Serialize)]
struct ApiResponse {
    code: i32,
    message: String,
    data: Option<Data>
}

#[get("/on")]
async fn on() -> Result<Json<ApiResponse>, anyhow::Error> {
    // load env vars
    dotenv().ok();

    let goove_api_key = std::env::var("GOVEE_API_KEY").expect("GOVEE_API_KEY must be set.").to_string();
    let goove_api_device= std::env::var("GOVEE_DEVICE_ID").expect("GOVEE_DEVICE_ID must be set").to_string();
    let goove_model= std::env::var("GOVEE_MODEL").expect("GOVEE_MODEL must be set").to_string();
    let command = Command {
        name: "turn".to_string(),
        value: "off".to_string(),
    };
    let govee_api_url = "https://developer-api.govee.com/v1/devices/control";

    let payload = json!({
        "device": goove_api_device,
        "model": goove_model,
        "cmd": command,
    });

    println!("payload: {}", payload);
    
    let client = reqwest::Client::new();

    let response = client
        .put(govee_api_url)
        .header("Govee-API-Key", goove_api_key)
        .json(&payload)
        .send()
        .await?;
        // .unwrap();
    let response_body: ApiResponse  = response.json().await?;
    Ok(Json(response_body))


    // println!("response: {:?}", response);
    // return response.json().await;
    // Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![on])
}
