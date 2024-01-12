use std::env::{self, var};

use govee_api::structs::govee::{GoveeCommand, PayloadBody};

use crate::constants::enums::{Device, OfficeDevices};

pub fn tv_light_setup(command: &str) -> PayloadBody {
    // TODO: Do I need this?
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

impl OfficeDevices {
    pub fn board_led() -> Self {
        let office_board_led_id =
            env::var("OFFICE_BOARD_LED_ID").expect("OFFICE_BOARD_LED_ID must be set");
        let office_board_led_model =
            env::var("OFFICE_BOARD_LED_MODEL").expect("OFFICE_BOARD_LED_MODEL must be set");
        let board_led = Device {
            device_id: office_board_led_id,
            model: office_board_led_model,
        };
        OfficeDevices::BoardLED(board_led)
    }

    pub fn standing_right_led() -> Self {
        let office_standing_right_led_id = env::var("OFFICE_STANDING_RIGHT_LED_ID")
            .expect("OFFICE_STANDING_RIGHT_LED_ID must be set");
        let office_standing_right_led_model = env::var("OFFICE_STANDING_RIGHT_LED_MODEL")
            .expect("OFFICE_STANDING_RIGHT_LED_MODEL must be set");
        let corner_led = Device {
            device_id: office_standing_right_led_id,
            model: office_standing_right_led_model,
        };
        OfficeDevices::StandingRightLED(corner_led)
    }

    pub fn standing_left_led() -> Self {
        let office_standing_left_led_id = env::var("OFFICE_STANDING_LEFT_LED_ID")
            .expect("OFFICE_STANDING_LEFT_LED_ID must be set");
        let office_standing_left_led_model =
            env::var("OFFICE_STANDING_LEFT_LED_MODEL").expect("OFFICE_STANDING_LEFT_LED_MODEL must be set");
        let corner_led = Device {
            device_id: office_standing_left_led_id,
            model: office_standing_left_led_model,
        };
        OfficeDevices::StandingLeftLED(corner_led)
    }

    pub fn table_led() -> Self {
        let office_table_led_id =
            env::var("OFFICE_TABLE_LED_ID").expect("OFFICE_TABLE_LED_ID must be set");
        let office_table_led_model =
            env::var("OFFICE_TABLE_LED_MODEL").expect("OFFICE_TABLE_LED_MODEL must be set");
        let table_led = Device {
            device_id: office_table_led_id,
            model: office_table_led_model,
        };
        OfficeDevices::TableLED(table_led)
    }

    pub fn window_led() -> Self {
        let office_window_led_id =
            env::var("OFFICE_WINDOW_LED_ID").expect("OFFICE_WINDOW_LED_ID must be set");
        let office_window_led_model =
            env::var("OFFICE_WINDOW_LED_MODEL").expect("OFFICE_WINDOW_LED_MODEL must be set");
        let window_led = Device {
            device_id: office_window_led_id,
            model: office_window_led_model,
        };
        OfficeDevices::WindowLED(window_led)
    }
    
    pub fn humidifier() -> Self {
        let office_humidifier_id=
            env::var("OFFICE_HUMIDIFIER_ID").expect("OFFICE_HUMIDIFIER_ID must be set");
        let office_humidifier_model =
            env::var("OFFICE_HUMIDIFIER_MODEL").expect("OFFICE_HUMIDIFIER_MODEL must be set");
        let humidifier = Device {
            device_id: office_humidifier_id,
            model: office_humidifier_model,
        };
        OfficeDevices::WindowLED(humidifier)
    }
    
}

pub fn office_setup(device: &OfficeDevices, command: &str) -> PayloadBody {
    let command = GoveeCommand {
        name: "turn".to_string(),
        value: command.to_string(),
    };
    match device {
        OfficeDevices::BoardLED(board_led) => PayloadBody {
            device: board_led.device_id.clone(),
            model: board_led.model.clone(),
            cmd: command,
        },
        OfficeDevices::TableLED(table_led) => PayloadBody {
            device: table_led.device_id.clone(),
            model: table_led.model.clone(),
            cmd: command,
        },
        OfficeDevices::WindowLED(window_led) => PayloadBody {
            device: window_led.device_id.clone(),
            model: window_led.model.clone(),
            cmd: command,
        },
        OfficeDevices::StandingRightLED(standing_right_led) => PayloadBody {
            device: standing_right_led.device_id.clone(),
            model: standing_right_led.model.clone(),
            cmd: command,
        },
        OfficeDevices::StandingLeftLED(standing_left_led) => PayloadBody {
            device: standing_left_led.device_id.clone(),
            model: standing_left_led.model.clone(),
            cmd: command,
        },
        OfficeDevices::Humidifier(humidifier) => PayloadBody {
            device: humidifier.device_id.clone(),
            model: humidifier.model.clone(),
            cmd: command,
        },
    }
}
