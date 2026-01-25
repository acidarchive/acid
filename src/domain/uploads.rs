use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum UploadType {
    Avatar,
    Banner,
}

impl UploadType {
    pub fn s3_prefix(&self) -> &'static str {
        match self {
            Self::Avatar => "avatars",
            Self::Banner => "banners",
        }
    }

    pub fn max_size_bytes(&self) -> u64 {
        match self {
            Self::Avatar => 2 * 1024 * 1024, // 2MB
            Self::Banner => 5 * 1024 * 1024, // 5MB
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, ToSchema)]
pub enum ContentType {
    #[serde(rename = "image/jpeg")]
    Jpeg,
    #[serde(rename = "image/png")]
    Png,
    #[serde(rename = "image/webp")]
    Webp,
    #[serde(rename = "image/gif")]
    Gif,
}

impl AsRef<str> for ContentType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::Webp => "image/webp",
            Self::Gif => "image/gif",
        }
    }
}
