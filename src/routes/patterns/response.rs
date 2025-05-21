use serde::Serialize;

#[derive(Serialize)]
pub struct PatternErrorResponse {
    pub status: String,
    pub message: String,
}
