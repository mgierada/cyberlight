#[cfg(test)]
pub mod tests {
    use crate::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_get_all_devices_handler() {
        let client = Client::untracked(crate::rocket()).expect("valid rocket instance");
        let response = client.get("/devices").dispatch();
        assert_eq!(response.status(), Status::Ok);
        // Add more assertions to validate the response body
    }

    #[test]
    fn test_get_status_for_all_devices() {
        let client = Client::untracked(crate::rocket()).expect("valid rocket instance");
        let response = client.get("/status").dispatch();
        assert_eq!(response.status(), Status::Ok);
        // Add more assertions to validate the response body
    }

    #[test]
    fn test_get_status_for_device() {
        let client = Client::untracked(crate::rocket()).expect("valid rocket instance");
        let response = client.get("/status/46:BA:A4:C1:38:1D:CF:A8/H6143").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let json_body = response.into_string().unwrap();
        let parsed_json: serde_json::Value = serde_json::from_str(&json_body).unwrap();
        assert!(parsed_json["status"].is_object());
        let response = parsed_json["status"].as_object().unwrap();
        assert!(response["device"].is_string());
        assert!(response["model"].is_string());
        assert!(response["properties"].is_array());
        let properties = response["properties"].as_array().unwrap();
        assert!(properties.len() > 0);
        let property = properties[0].as_object().unwrap();
        println!("{:?}", property);
        // assert!(property["online"].is_boolean());
        // assert!(property["powerState"].is_string());
        // assert!(property["brightness"].is_number());
        // assert!(property["color"].is_string());
    }
}
