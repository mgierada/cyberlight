use serde::{Deserialize, Serialize};
use std::env::var;

#[derive(Serialize)]
pub struct PayloadBody {
    pub device: String,
    pub model: String,
    pub cmd: GoveeCommand,
}

#[derive(Serialize, Deserialize)]
pub struct GoveeCommand {
    pub name: String,
    pub value: String,
}

pub fn tv_light_setup(command: &str) -> PayloadBody {
    let goove_api_device =
        var("GOVEE_DEVICE_ID_TV_LIGHT").expect("GOVEE_DEVICE_ID_TV_LIGHT must be set");
    let goove_model = var("GOVEE_MODEL_TV_LIGHT").expect("GOVEE_MODEL_TV_LIGHT must be set");
    let command = GoveeCommand {
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
    let goove_api_device =
        var("GOVEE_DEVICE_ID_OFFICE_LIGHT").expect("GOVEE_DEVICE_ID_OFFICE_LIGHT must be set");
    let goove_model =
        var("GOVEE_MODEL_OFFICE_LIGHT").expect("GOVEE_MODEL_OFFICE_LIGHT must be set");
    let command = GoveeCommand {
        name: "turn".to_string(),
        value: command.to_string(),
    };
    PayloadBody {
        device: goove_api_device,
        model: goove_model,
        cmd: command,
    }
}
