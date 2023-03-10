use dotenv::dotenv;
use reqwest::Error;
use serde::{Serialize, Deserialize};
use serde_json::{json, Result};

#[derive(Serialize, Deserialize, Debug)]
struct Command {
    name: String,
    value: String,
}
fn main() -> Result<()> {
    // load env vars
    dotenv().ok();

    let goove_api_token = std::env::var("GOVEE_API_KEY").expect("GOVEE_API_KEY must be set.").to_string();
    let goove_api_device= std::env::var("GOVEE_DEVICE_ID").expect("GOVEE_DEVICE_ID").to_string();
    let goove_model= std::env::var("GOVEE_MODEL").expect("GOVEE_MODEL").to_string();
    let command = Command {
        name: "turn".to_string(),
        value: "on".to_string(),
    };
    let govee_api_url = "https://developer-api.govee.com/v1/devices";

    let payload = json!({
        "device": goove_api_device,
        "model": goove_model,
        "cmd": command,
    });

    println!("payload: {}", payload);

    Ok(())
}
