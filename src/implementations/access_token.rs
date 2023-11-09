use crate::error_handlers::error_implementations::AuthError;
use crate::ACCESS_TOKEN;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

#[derive(Debug, PartialEq)]
pub struct Token(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = AuthError;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_token) = auth_header {
            if auth_token == ACCESS_TOKEN.to_string() {
                Outcome::Success(Token(auth_token.to_string()))
            } else {
                Outcome::Error((
                    Status::Unauthorized,
                    AuthError {
                        error: "Invalid Authorization token".to_string(),
                    },
                ))
            }
        } else {
            Outcome::Error((
                Status::Unauthorized,
                AuthError {
                    error: "Missing Authorization header".to_string(),
                },
            ))
        }
    }
}
