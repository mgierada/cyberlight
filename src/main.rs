#[macro_use]
extern crate rocket;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use routes::office_lamp_routes::{office_off_handler, office_on_handler};
use routes::tv_lamp_routes::{tv_off_handler, tv_on_handler};
pub mod routes;
pub mod services;

#[derive(Serialize)]
pub struct PayloadBody {
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
        std::env::var("GOVEE_DEVICE_ID_TV_LIGHT").expect("GOVEE_DEVICE_ID_TV_LIGHT must be set");
    let goove_model =
        std::env::var("GOVEE_MODEL_TV_LIGHT").expect("GOVEE_MODEL_TV_LIGHT must be set");
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

fn office_light_setup(command: &str) -> PayloadBody {
    let goove_api_device = std::env::var("GOVEE_DEVICE_ID_OFFICE_LIGHT")
        .expect("GOVEE_DEVICE_ID_OFFICE_LIGHT must be set");
    let goove_model =
        std::env::var("GOVEE_MODEL_OFFICE_LIGHT").expect("GOVEE_MODEL_OFFICE_LIGHT must be set");
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/tv", routes![tv_on_handler, tv_off_handler])
        .mount("/office", routes![office_on_handler, office_off_handler])
}
