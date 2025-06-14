use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub details: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct QueryRequest {
    #[validate(length(
        min = 10,
        max = 1000,
        message = "Query must be between 10 and 1000 characters"
    ))]
    pub query: String,
}

#[derive(Debug, Serialize)]
pub struct QuerySuccess {
    pub message: String,
    pub query: String,
    pub response: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum QueryResponse {
    Success(QuerySuccess),
    Error(ErrorResponse),
}
