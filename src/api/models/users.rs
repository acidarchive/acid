use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    #[schema(example = "avatars/550e8400-e29b-41d4-a716-446655440000/f47ac10b-58cc...")]
    pub avatar_key: Option<String>,
    #[schema(example = "banners/550e8400-e29b-41d4-a716-446655440000/f47ac10b-58cc...")]
    pub banner_key: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub user_id: Uuid,
    #[schema(example = "https://bucket.s3.region.amazonaws.com/avatars/user-id")]
    pub avatar_url: Option<String>,
    #[schema(example = "https://bucket.s3.region.amazonaws.com/banners/user-id")]
    pub banner_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
