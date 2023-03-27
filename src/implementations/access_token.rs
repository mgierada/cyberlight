use crate::ACCESS_TOKEN;
use rocket::http::{ContentType, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::response::Responder;
use rocket::{Request, Response};
use serde::{Deserialize, Serialize};

// #[derive(Debug, Deserialize, PartialEq)]
// pub struct Token(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResp {
    message: String,
}

#[derive(Debug)]
struct ApiResponse {
    error: String,
}

// impl<'r> Responder<'r> for ApiResponse {
//     fn respond_to(self, req: &Request) -> response::Result<'r> {
//         Response::build_from(self.json.respond_to(&req).unwrap())
//             .status(self.status)
//             .header(ContentType::JSON)
//             .ok()
//     }
// }
//

// impl<'r> Responder<'r, 'static> for ApiResponse {
//     fn respond_to(self, _: &Request<'_>) -> Result<Response<'static>, Status> {
//         let json = serde_json::json!({ "error": self.error });
//         let response = Response::build()
//             .header(ContentType::JSON)
//             .status(Status::InternalServerError)
//             .sized_body(
//                 json.to_string().len(),
//                 std::io::Cursor::new(json.to_string()),
//             )
//             .finalize();
//         Ok(response)
//     }
// }
//
// #[catch(500)]
// pub fn internal_server_error() -> ApiResponse {
//     ApiResponse {
//         error: "Internal Server Error".to_string(),
//     }
// }

#[derive(Debug, Deserialize, PartialEq)]
pub struct Token(String);

#[derive(Debug, Serialize)]
pub struct AuthError {
    error: String,
}

impl<'r> Responder<'r, 'static> for AuthError {
    fn respond_to(self, _: &rocket::Request<'_>) -> rocket::response::Result<'static> {
        let json = serde_json::to_string(&self).unwrap();
        Ok(Response::build()
            .status(Status::Unauthorized)
            .header(ContentType::JSON)
            .sized_body(json.len(), std::io::Cursor::new(json))
            .finalize())
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");

        if let Some(auth_token) = auth_header {
            if auth_token == ACCESS_TOKEN.to_string(){
                Outcome::Success(Token(auth_token.to_string()))
            } else {
                Outcome::Failure((
                    Status::Unauthorized,
                    AuthError {
                        error: "Invalid Authorization token".to_string(),
                    },
                ))
            }
        } else {
            Outcome::Failure((
                Status::Unauthorized,
                AuthError {
                    error: "Missing Authorization header".to_string(),
                },
            ))
        }
    }
}
//
// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for Token {
//     type Error = ();
//
//     async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
//         // Attempt to get the token from the `Authorization` header
//         match req.headers().get_one("Authorization") {
//             Some(token_str) => {
//                 // If the header is present, return a `Token` struct with the token string
//                 let token = Token(token_str.to_string());
//                 if token_str != ACCESS_TOKEN.to_string() {
//                     // If the token is not valid, return a 401 Unauthorized
//                     Outcome::Failure((Status::Unauthorized, ()))
//                     // Outcome::Failure((Status::Unauthorized, ())).::<Token>
//                 } else {
//                     Outcome::Success(token)
//                 }
//             }
//             None => {
//                 // If the header is not present, return a 401 Unauthorized
//                 //
//                 // Outcome::Failure((Status::Unauthorized, ()))
//                 Outcome::Failure((Status::Unauthorized, Json(ErrorResp {
//                     message: "Unauthorized".to_string(),
//                 })))
//             }
//         }
//     }
// }
