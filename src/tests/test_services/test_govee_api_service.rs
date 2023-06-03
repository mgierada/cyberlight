#[cfg(test)]
mod tests {
    use crate::services::{
        govee_api_service::sent_put_request,
        light_setup_service::{GoveeCommand, PayloadBody},
    };
    use mockito;

    #[tokio::test]
    async fn test_sent_put_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new();

        // Use one of these addresses to configure your client
        let url = server.url();
        let govee_api_key = "1234567890";
        let mock_endpoint = server
            .mock("put", "/v1/devices/control")
            .match_header("govee-api-key", govee_api_key)
            .with_status(200)
            .create();

        // Arrange
        let govee_root_url = url;
        let command = GoveeCommand {
            name: "turn".to_string(),
            value: "on".to_string(),
        };
        let payload = PayloadBody {
            device: "device_id".to_string(),
            model: "model_id".to_string(),
            cmd: command,
        };

        // Act
        sent_put_request(&govee_root_url, govee_api_key, payload).await;
        mock_endpoint.assert();
    }
}

// async fn test_sent_put_request() {
//     let mock_endpoint = mock("put", "/v1/devices/control")
//         .match_header("govee-api-key", "test_key")
//         .with_status(200)
//         .create();
//     let payload = payloadbody {
//         device: "test_device".to_string(),
//         model: "test_model".to_string(),
//         cmd: "test_cmd".to_string(),
//         value: "test_value".to_string(),
//     };
//     sent_put_request("http://localhost:1234", "test_key", payload).await;
//     mock_endpoint.assert();
// }

// Assert
// Check that the request was successful
// and that the response status code is 2xx

// #[test]
// fn test_payload_body_serialization() {
//     // Arrange
//     let payload = PayloadBody {
//         device: "device_id".to_string(),
//         cmd: "turn_on".to_string(),
//         value: None,
//     };
//
//     // Act
//     let payload_json = json!(payload);
//
//     // Assert
//     // Check that the payload JSON is correctly serialized
//     // and contains the expected fields and values
// }
