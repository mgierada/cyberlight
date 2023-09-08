use std::env::var;

use govee_api::structs::govee::{GoveeCommand, PayloadBody};

use crate::constants::enums::{Device, OfficeDevices};

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

pub fn office_light_setup(device: &OfficeDevices, command: &str) -> PayloadBody {
    let office_corner_light_id =
        var("OFFICE_CORNER_LIGHT_ID").expect("OFFICE_CORNER_LIGHT_ID must be set");
    let office_corner_light_model =
        var("OFFICE_CORNER_LIGHT_MODEL").expect("OFFICE_CORNER_LIGHT_MODEL must be set");
    let office_table_led_id = var("OFFICE_TABLE_LED_ID").expect("OFFICE_TABLE_LED_ID must be set");
    let office_table_led_model =
        var("OFFICE_TABLE_LED_MODEL").expect("OFFICE_TABLE_LED_MODEL must be set");

    let command = GoveeCommand {
        name: "turn".to_string(),
        value: command.to_string(),
    };

    match device {
        OfficeDevices::CornerLED(_) => PayloadBody {
            device: office_corner_light_id,
            model: office_corner_light_model,
            cmd: command,
        },
        OfficeDevices::TableLED(_) => PayloadBody {
            device: office_table_led_id,
            model: office_table_led_model,
            cmd: command,
        },
    }
}
