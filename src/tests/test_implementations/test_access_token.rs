#[cfg(test)]
mod tests {
    use crate::rocket;
    use rocket::http::{Header, Status};
    use rocket::local::blocking::Client;
    use std::env;

    #[test]
    #[ignore = "requires ACCESS_TOKEN and GOVEE_API_KEY environment variables"]
    fn test_valid_authorization_token() {
        // This test verifies that a valid token allows access
        let client = Client::untracked(rocket()).expect("valid rocket instance");
        let token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN must be set");
        let response = client
            .get("/standing/on")
            .header(Header::new("Authorization", token))
            .dispatch();
        // Note: This will fail if we don't have the Govee API key set, 
        // but it won't fail with 401 Unauthorized
        assert_ne!(response.status(), Status::Unauthorized);
    }

    #[test]
    #[ignore = "requires ACCESS_TOKEN environment variable"]
    fn test_missing_authorization_header() {
        let client = Client::untracked(rocket()).expect("valid rocket instance");
        let response = client.get("/standing/on").dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
        let body = response.into_string().expect("response into string");
        assert!(body.contains("Missing Authorization header"));
    }

    #[test]
    #[ignore = "requires ACCESS_TOKEN environment variable"]
    fn test_invalid_authorization_token() {
        let client = Client::untracked(rocket()).expect("valid rocket instance");
        let response = client
            .get("/standing/on")
            .header(Header::new("Authorization", "invalid_token"))
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
        let body = response.into_string().expect("response into string");
        assert!(body.contains("Invalid Authorization token"));
    }
}
