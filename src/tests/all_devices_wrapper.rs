#[cfg(test)]
mod tests {
    use crate::{
        services::govee_api_service::{ColorTem, ColorTemRange, GoveeDevice, Properties},
        wrappers::all_devices_wrapper::wrap_devices,
    };

    use super::*;

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
}
