use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserErrorResponse {
    pub status: String,
    pub message: String,
}
