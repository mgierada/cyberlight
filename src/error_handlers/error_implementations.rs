use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::Response;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthError {
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotFoundError {
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerError {
    pub error: String,
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

impl<'r> Responder<'r, 'static> for NotFoundError {
    fn respond_to(self, _: &rocket::Request<'_>) -> rocket::response::Result<'static> {
        let json = serde_json::to_string(&self).unwrap();
        Ok(Response::build()
            .status(Status::NotFound)
            .header(ContentType::JSON)
            .sized_body(json.len(), std::io::Cursor::new(json))
            .finalize())
    }
}

impl<'r> Responder<'r, 'static> for ServerError {
    fn respond_to(self, _: &rocket::Request<'_>) -> rocket::response::Result<'static> {
        let json = serde_json::to_string(&self).unwrap();
        Ok(Response::build()
            .status(Status::InternalServerError)
            .header(ContentType::JSON)
            .sized_body(json.len(), std::io::Cursor::new(json))
            .finalize())
    }
}
