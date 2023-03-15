use serde::{Deserialize, Serialize};

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

pub fn tv_light_setup(command: &str) -> PayloadBody {
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

pub fn office_light_setup(command: &str) -> PayloadBody {
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
