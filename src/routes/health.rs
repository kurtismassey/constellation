use axum::{
    response::IntoResponse,
    http::StatusCode,
    Json
};
use serde_json::json;

pub async fn get() -> impl IntoResponse {
    (
    StatusCode::OK, Json(json!({
        "status": "online",
        "message": "Constellation is running normally!"
    }))
    )
}