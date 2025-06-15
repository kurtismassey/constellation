use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub async fn get() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "status": "online",
            "message": "Constellation is running normally!"
        })),
    )
}
