use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::Deserialize;

use crate::ACCESS_TOKEN;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Token(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Attempt to get the token from the `Authorization` header
        match req.headers().get_one("Authorization") {
            Some(token_str) => {
                // If the header is present, return a `Token` struct with the token string
                let token = Token(token_str.to_string());
                if token_str != ACCESS_TOKEN.to_string() {
                    // If the token is not valid, return a 401 Unauthorized
                    Outcome::Failure((Status::Unauthorized, ()))
                    // Outcome::Failure((Status::Unauthorized, ())).::<Token>
                } else {
                    Outcome::Success(token)
                }
            }
            None => {
                // If the header is not present, return a 401 Unauthorized
                Outcome::Failure((Status::Unauthorized, ()))
            }
        }
    }
}
