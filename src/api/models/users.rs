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
    #[schema(example = "26f29224-6001-702f-25dc-6d5c1b750f51")]
    pub user_id: Uuid,
    #[schema(example = "username")]
    pub username: String,
    #[schema(example = "https://bucket.s3.region.amazonaws.com/avatars/user-id")]
    pub avatar_url: Option<String>,
    #[schema(example = "https://bucket.s3.region.amazonaws.com/banners/user-id")]
    pub banner_url: Option<String>,
    #[schema(example = "2023-10-01T12:00:00Z")]
    pub created_at: DateTime<Utc>,
    #[schema(example = "2023-10-01T12:00:00Z")]
    pub updated_at: DateTime<Utc>,
}
