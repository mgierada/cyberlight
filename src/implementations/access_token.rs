use crate::ACCESS_TOKEN;
use rocket::http::{ContentType, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::{Request, Response};
use serde::{Deserialize, Serialize};



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


#[catch(default)]
pub async fn auth_error_catcher() -> Json<AuthError> {
    let auth_error = AuthError {
        error: "Invalid Authorization token".to_string(),
    };
    Json(auth_error)
}

