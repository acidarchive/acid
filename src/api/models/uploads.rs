use crate::domain::{ContentType, UploadType};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct PresignRequest {
    #[schema(example = "avatar")]
    pub upload_type: UploadType,
    #[schema(example = "image/png")]
    pub content_type: ContentType,
    #[schema(example = 102400)]
    pub content_length: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PresignResponse {
    #[schema(example = "https://bucket.s3.region.amazonaws.com/avatars/user-id?X-Amz-...")]
    pub upload_url: String,
    #[schema(
        example = "avatars/550e8400-e29b-41d4-a716-446655440000/f47ac10b-58cc-4372-a567-0e02b2c3d479"
    )]
    pub key: String,
}
