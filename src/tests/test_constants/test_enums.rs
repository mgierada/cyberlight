#[cfg(test)]
mod tests {
    use crate::constants::enums::{Device, OfficeDevices};

    #[test]
    fn test_device_creation() {
        let device = Device {
            device_id: "test_id".to_string(),
            model: "test_model".to_string(),
        };
        assert_eq!(device.device_id, "test_id");
        assert_eq!(device.model, "test_model");
    }

    #[test]
    fn test_office_devices_variants() {
        let table_led = OfficeDevices::TableLED(Device {
            device_id: "table_id".to_string(),
            model: "table_model".to_string(),
        });
        match table_led {
            OfficeDevices::TableLED(device) => {
                assert_eq!(device.device_id, "table_id");
                assert_eq!(device.model, "table_model");
            }
            _ => panic!("Expected TableLED variant"),
        }

        let standing_right = OfficeDevices::StandingRightLED(Device {
            device_id: "right_id".to_string(),
            model: "right_model".to_string(),
        });
        match standing_right {
            OfficeDevices::StandingRightLED(device) => {
                assert_eq!(device.device_id, "right_id");
                assert_eq!(device.model, "right_model");
            }
            _ => panic!("Expected StandingRightLED variant"),
        }
    }

    #[test]
    fn test_all_office_device_variants_can_be_created() {
        let devices = vec![
            OfficeDevices::TableLED(Device {
                device_id: "1".to_string(),
                model: "m1".to_string(),
            }),
            OfficeDevices::StandingRightLED(Device {
                device_id: "2".to_string(),
                model: "m2".to_string(),
            }),
            OfficeDevices::StandingLeftLED(Device {
                device_id: "3".to_string(),
                model: "m3".to_string(),
            }),
            OfficeDevices::WindowLED(Device {
                device_id: "4".to_string(),
                model: "m4".to_string(),
            }),
            OfficeDevices::BoardLED(Device {
                device_id: "5".to_string(),
                model: "m5".to_string(),
            }),
            OfficeDevices::Humidifier(Device {
                device_id: "6".to_string(),
                model: "m6".to_string(),
            }),
        ];
        // Test that all 6 variants can be created
        assert_eq!(devices.len(), 6);
    }
}
