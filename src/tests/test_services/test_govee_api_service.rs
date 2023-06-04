#[cfg(test)]
mod tests {
    use crate::services::{
        govee_api_service::{get_all_devices, sent_put_request},
        light_setup_service::{GoveeCommand, PayloadBody},
    };
    use mockito;

    #[tokio::test]
    async fn test_sent_put_request() {
        let mut server = mockito::Server::new();
        let govee_root_url = server.url();
        let govee_api_key = "1234567890";
        let mock_endpoint = server
            .mock("put", "/v1/devices/control")
            .match_header("govee-api-key", govee_api_key)
            .with_status(200)
            .create();
        let command = GoveeCommand {
            name: "turn".to_string(),
            value: "on".to_string(),
        };
        let payload = PayloadBody {
            device: "device_id".to_string(),
            model: "model_id".to_string(),
            cmd: command,
        };
        sent_put_request(&govee_root_url, &govee_api_key, payload).await;
        mock_endpoint.assert();
    }

    #[tokio::test]
    async fn test_get_all_devices() {
        let mut server = mockito::Server::new();
        let govee_root_url = server.url();
        let govee_api_key = "1234567890";
        let mock_endpoint = server
            .mock("get", "/v1/devices")
            .match_header("govee-api-key", govee_api_key)
            .with_status(200)
            .with_body(
                r#"{
                    "code": 200,
                    "message": "Success",
                    "devices": [
                        {
                            "device": "device_id",
                            "model": "model_id",
                            "deviceName": "device_name",
                            "controllable": true,
                            "retrievable": true,
                            "supportCmds": [
                                "turn",
                                "brightness",
                                "color",
                                "colorTem"
                            ],
                            "properties": {
                                "colorTem": {
                                    "range": {
                                        "min": 2000,
                                        "max": 9000
                                    }
                                }
                            }
                        }
                    ]
                }"#,
            )
            .create();
        get_all_devices(&govee_root_url, &govee_api_key).await;
        mock_endpoint.assert();
    }
}
