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
