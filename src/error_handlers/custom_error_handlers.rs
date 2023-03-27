use rocket::serde::json::Json;

use crate::implementations::access_token::AuthError;

#[catch(401)]
pub async fn ununauthorized() -> Json<AuthError> {
    let auth_error = AuthError {
        error: "Invalid Authorization token".to_string(),
    };
    Json(auth_error)
}

#[catch(404)]
pub fn not_found() -> Json<AuthError>{
    let auth_error = AuthError {
        error: "Page not found".to_string(),
    };
    Json(auth_error)
}
