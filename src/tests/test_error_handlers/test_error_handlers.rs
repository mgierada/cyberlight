#[cfg(test)]
mod tests {
    use crate::error_handlers::error_implementations::{AuthError, NotFoundError};
    use crate::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_unauthorized_handler() {
        let client = Client::untracked(rocket()).expect("valid rocket instance");
        let response = client.get("/office/on").dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
        let body = response.into_string().expect("response into string");
        let error: AuthError = serde_json::from_str(&body).expect("deserialize error");
        assert_eq!(error.error, "Invalid Authorization token");
    }

    #[test]
    fn test_not_found_handler() {
        let client = Client::untracked(rocket()).expect("valid rocket instance");
        let response = client.get("/not_found_url").dispatch();
        assert_eq!(response.status(), Status::NotFound);
        let body = response.into_string().expect("response into string");
        let error: NotFoundError = serde_json::from_str(&body).expect("deserialize error");
        assert_eq!(error.error, "Page not found");
    }
}
