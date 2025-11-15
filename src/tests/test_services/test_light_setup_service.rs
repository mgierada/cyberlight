#[cfg(test)]
mod tests {
    use crate::constants::enums::{Device, OfficeDevices};
    use crate::services::light_setup_service::office_setup;

    #[test]
    fn test_office_setup_creates_payload_with_on_command() {
        let device = OfficeDevices::BoardLED(Device {
            device_id: "test_device_id".to_string(),
            model: "test_model".to_string(),
        });
        let payload = office_setup(&device, "on");
        
        assert_eq!(payload.device, "test_device_id");
        assert_eq!(payload.model, "test_model");
        assert_eq!(payload.cmd.name, "turn");
        assert_eq!(payload.cmd.value, "on");
    }

    #[test]
    fn test_office_setup_creates_payload_with_off_command() {
        let device = OfficeDevices::TableLED(Device {
            device_id: "table_device_id".to_string(),
            model: "table_model".to_string(),
        });
        let payload = office_setup(&device, "off");
        
        assert_eq!(payload.device, "table_device_id");
        assert_eq!(payload.model, "table_model");
        assert_eq!(payload.cmd.name, "turn");
        assert_eq!(payload.cmd.value, "off");
    }

    #[test]
    fn test_office_setup_handles_all_device_types() {
        let devices = vec![
            OfficeDevices::BoardLED(Device {
                device_id: "board_id".to_string(),
                model: "board_model".to_string(),
            }),
            OfficeDevices::TableLED(Device {
                device_id: "table_id".to_string(),
                model: "table_model".to_string(),
            }),
            OfficeDevices::WindowLED(Device {
                device_id: "window_id".to_string(),
                model: "window_model".to_string(),
            }),
            OfficeDevices::StandingRightLED(Device {
                device_id: "right_id".to_string(),
                model: "right_model".to_string(),
            }),
            OfficeDevices::StandingLeftLED(Device {
                device_id: "left_id".to_string(),
                model: "left_model".to_string(),
            }),
            OfficeDevices::Humidifier(Device {
                device_id: "humidifier_id".to_string(),
                model: "humidifier_model".to_string(),
            }),
        ];

        for device in devices {
            let payload = office_setup(&device, "on");
            assert_eq!(payload.cmd.name, "turn");
            assert_eq!(payload.cmd.value, "on");
            assert!(!payload.device.is_empty());
            assert!(!payload.model.is_empty());
        }
    }
}
