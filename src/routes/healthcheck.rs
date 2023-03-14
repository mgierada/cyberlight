use rocket::serde::json::Json;

#[get("/healthcheck")]
pub async fn healthcheck_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "healthy"}))
}
