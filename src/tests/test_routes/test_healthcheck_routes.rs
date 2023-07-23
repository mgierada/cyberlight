#[cfg(test)]
mod tests {
    use crate::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_healthcheck_handler() {
        let client = Client::untracked(rocket()).expect("valid rocket instance");
        let response = client.get("/healthcheck").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().expect("valid response body");
        let json: serde_json::Value = serde_json::from_str(&body).expect("valid JSON");
        assert_eq!(json["status"], "healthy");
        assert!(json["deployed_version"].is_string());
    }
}
