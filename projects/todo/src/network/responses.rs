use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct SuccessResponse<T> {
    pub message: String,
    pub contents: T,
    pub has_next: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub message: String,
}
