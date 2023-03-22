
use rocket::Request;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::Json;
use serde::Deserialize;

use crate::services::govee_api_service::sent_put_request;
use crate::services::light_setup_service::tv_light_setup;
use crate::{GOVEE_API_KEY, GOVEE_ROOT_URL};

#[derive(Debug, Deserialize)]
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
                Outcome::Success(token)
            }
            None => {
                // If the header is not present, return a 401 Unauthorized response
                Outcome::Failure((Status::Unauthorized, ()))
            }
        }
    }
}

#[get("/on")]
pub async fn tv_on_handler(token: Token) -> Json<serde_json::Value> {
   // require access header to be present


    let payload = tv_light_setup("on");
    sent_put_request(&GOVEE_ROOT_URL, &GOVEE_API_KEY, payload).await;
    Json(serde_json::json!({"device": "tv_light", "status": "on"}))
}

#[get("/off")]
pub async fn tv_off_handler() -> Json<serde_json::Value> {
    let payload = tv_light_setup("off");
    sent_put_request(&GOVEE_ROOT_URL, &GOVEE_API_KEY, payload).await;
    Json(serde_json::json!({"device": "tv_light", "status": "off"}))
}
