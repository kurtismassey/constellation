use crate::models::{ErrorResponse, QueryRequest, QueryResponse, QuerySuccess};
use axum::{http::StatusCode, response::IntoResponse, Json};
use validator::Validate;

pub async fn post(Json(request): Json<QueryRequest>) -> impl IntoResponse {
    match request.validate() {
        Ok(_) => (
            StatusCode::OK,
            Json(QueryResponse::Success(QuerySuccess {
                message: "Query received".to_string(),
                query: request.query,
            })),
        ),
        Err(validation_errors) => {
            let error_messages: Vec<String> = validation_errors
                .field_errors()
                .iter()
                .flat_map(|(_, errors)| {
                    errors.iter().map(|error| {
                        error
                            .message
                            .as_ref()
                            .map(|error_message| error_message.to_string())
                            .unwrap_or_else(|| "Invalid input".to_string())
                    })
                })
                .collect();

            tracing::error!("Validation failed: {:?}", error_messages);
            (
                StatusCode::BAD_REQUEST,
                Json(QueryResponse::Error(ErrorResponse {
                    error: "Bad request".to_string(),
                    details: error_messages,
                })),
            )
        }
    }
}
