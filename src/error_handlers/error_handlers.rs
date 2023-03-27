use rocket::serde::json::Json;

use super::error_implementations::{AuthError, NotFoundError};

#[catch(401)]
pub async fn ununauthorized() -> Json<AuthError> {
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
