#[cfg(test)]
mod tests {
    use crate::error_handlers::error_implementations::{AuthError, NotFoundError, ServerError};

    #[test]
    fn test_auth_error_serialization() {
        let error = AuthError {
            error: "Unauthorized access".to_string(),
        };
        let json = serde_json::to_string(&error).expect("serialize auth error");
        assert!(json.contains("Unauthorized access"));
        assert!(json.contains("error"));
    }

    #[test]
    fn test_auth_error_deserialization() {
        let json = r#"{"error":"Invalid token"}"#;
        let error: AuthError = serde_json::from_str(json).expect("deserialize auth error");
        assert_eq!(error.error, "Invalid token");
    }

    #[test]
    fn test_not_found_error_serialization() {
        let error = NotFoundError {
            error: "Resource not found".to_string(),
        };
        let json = serde_json::to_string(&error).expect("serialize not found error");
        assert!(json.contains("Resource not found"));
        assert!(json.contains("error"));
    }

    #[test]
    fn test_not_found_error_deserialization() {
        let json = r#"{"error":"Page not found"}"#;
        let error: NotFoundError = serde_json::from_str(json).expect("deserialize not found error");
        assert_eq!(error.error, "Page not found");
    }

    #[test]
    fn test_server_error_serialization() {
        let error = ServerError {
            error: "Internal server error".to_string(),
        };
        let json = serde_json::to_string(&error).expect("serialize server error");
        assert!(json.contains("Internal server error"));
        assert!(json.contains("error"));
    }

    #[test]
    fn test_server_error_deserialization() {
        let json = r#"{"error":"Something went wrong"}"#;
        let error: ServerError = serde_json::from_str(json).expect("deserialize server error");
        assert_eq!(error.error, "Something went wrong");
    }
}
