use rocket::serde::json::Json;

use super::error_implementations::{AuthError, NotFoundError, ServerError};

#[catch(401)]
pub fn ununauthorized() -> Json<AuthError> {
    let auth_error = AuthError {
        error: "Invalid Authorization token".to_string(),
    };
    Json(auth_error)
}

#[catch(404)]
pub fn not_found() -> Json<NotFoundError> {
    let not_found_error = NotFoundError {
        error: "Page not found".to_string(),
    };
    Json(not_found_error)
}

#[catch(500)]
pub fn server_error() -> Json<ServerError> {
    let server_error = ServerError {
        error: "Something unexpected occurred".to_string(),
    };
    Json(server_error)
}
