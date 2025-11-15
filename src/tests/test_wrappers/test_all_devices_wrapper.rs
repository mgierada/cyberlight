#[cfg(test)]
mod tests {
    use crate::wrappers::all_devices_wrapper::{
        wrap_device_status, wrap_devices, wrap_model_and_devices, DeviceStatus, ModelAndDevice,
    };
    use govee_api::structs::govee::{
        ColorTem, ColorTemRange, GoveeDataDeviceStatus, GoveeDevice, GoveeDeviceProperty,
        Properties,
    };

    #[test]
    fn test_wrap_devices() {
        let devices = vec![
            GoveeDevice {
                deviceName: "Device 1".to_string(),
                device: "device1".to_string(),
                model: "model1".to_string(),
                controllable: true,
                retrievable: true,
                supportCmds: vec![],
                properties: Properties {
                    colorTem: ColorTem {
                        range: ColorTemRange { min: 0, max: 0 },
                    },
                },
            },
            GoveeDevice {
                deviceName: "Device 2".to_string(),
                device: "device2".to_string(),
                model: "model2".to_string(),
                controllable: false,
                retrievable: true,
                supportCmds: vec![],
                properties: Properties {
                    colorTem: ColorTem {
                        range: ColorTemRange { min: 0, max: 0 },
                    },
                },
            },
        ];
        let wrapped_devices = wrap_devices(devices);
        assert_eq!(wrapped_devices.len(), 2);
        assert_eq!(wrapped_devices[0].deviceName, "Device 1");
        assert_eq!(wrapped_devices[0].device, "device1");
        assert_eq!(wrapped_devices[0].model, "model1");
        assert_eq!(wrapped_devices[0].controllable, true);
        assert_eq!(wrapped_devices[0].retrievable, true);
        assert_eq!(wrapped_devices[1].deviceName, "Device 2");
        assert_eq!(wrapped_devices[1].device, "device2");
        assert_eq!(wrapped_devices[1].model, "model2");
        assert_eq!(wrapped_devices[1].controllable, false);
        assert_eq!(wrapped_devices[1].retrievable, true);
    }

    #[test]
    fn test_wrap_model_and_devices() {
        let devices = vec![
            GoveeDevice {
                deviceName: "Device 1".to_string(),
                device: "ABC123".to_string(),
                model: "Model 1".to_string(),
                controllable: true,
                retrievable: true,
                supportCmds: vec![],
                properties: Properties {
                    colorTem: ColorTem {
                        range: ColorTemRange { min: 0, max: 0 },
                    },
                },
            },
            GoveeDevice {
                deviceName: "Device 2".to_string(),
                device: "DEF456".to_string(),
                model: "Model 2".to_string(),
                controllable: true,
                retrievable: true,
                supportCmds: vec![],
                properties: Properties {
                    colorTem: ColorTem {
                        range: ColorTemRange { min: 0, max: 0 },
                    },
                },
            },
        ];

        let expected_output = vec![
            ModelAndDevice {
                deviceName: "Device 1".to_string(),
                device: "ABC123".to_string(),
                model: "Model 1".to_string(),
            },
            ModelAndDevice {
                deviceName: "Device 2".to_string(),
                device: "DEF456".to_string(),
                model: "Model 2".to_string(),
            },
        ];

        assert_eq!(wrap_model_and_devices(devices), expected_output);
    }

    #[test]
    fn test_wrap_device_status() {
        let device = GoveeDataDeviceStatus {
            device: "device1".to_string(),
            model: "model1".to_string(),
            properties: vec![
                GoveeDeviceProperty::Online(true),
                GoveeDeviceProperty::PowerState("on".to_string()),
                GoveeDeviceProperty::Brightness(50),
            ],
        };
        let expected = DeviceStatus {
            device: "device1".to_string(),
            model: "model1".to_string(),
            properties: vec![
                GoveeDeviceProperty::Online(true),
                GoveeDeviceProperty::PowerState("on".to_string()),
            ],
        };
        assert_eq!(wrap_device_status(device), expected);
    }
}
