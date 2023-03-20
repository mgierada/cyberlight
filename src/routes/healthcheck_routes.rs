use rocket::serde::json::Json;
use std::process::Command;

#[get("/healthcheck")]
pub async fn healthcheck_handler() -> Json<serde_json::Value> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    Json(serde_json::json!({"status": "healthy", "deployed_version": git_hash}))
}
