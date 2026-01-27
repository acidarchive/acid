use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum S3ClientError {
    #[error("Failed to generate presigned URL: {0}")]
    PresignError(String),
}

pub struct S3Client {
    client: aws_sdk_s3::Client,
    bucket: String,
    region: String,
}

impl S3Client {
    pub async fn new(region: String, bucket: String, endpoint_url: Option<String>) -> Self {
        let region = aws_config::Region::new(region);
        let mut config_loader =
            aws_config::defaults(aws_config::BehaviorVersion::latest()).region(region.clone());

        if let Some(endpoint) = endpoint_url {
            config_loader = config_loader.endpoint_url(endpoint);
        }

        let sdk_config = config_loader.load().await;
        let client = aws_sdk_s3::Client::new(&sdk_config);

        Self {
            client,
            bucket,
            region: region.to_string(),
        }
    }

    pub async fn presign_put(
        &self,
        key: &str,
        content_type: &str,
        content_length: u64,
        expires_in: Duration,
    ) -> Result<String, S3ClientError> {
        let presigning_config = aws_sdk_s3::presigning::PresigningConfig::expires_in(expires_in)
            .map_err(|e| {
                S3ClientError::PresignError(format!("Failed to create presigning config: {e}"))
            })?;

        let presigned_request = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .content_type(content_type)
            .content_length(content_length as i64)
            .presigned(presigning_config)
            .await
            .map_err(|e| S3ClientError::PresignError(format!("Failed to presign request: {e}")))?;

        Ok(presigned_request.uri().to_string())
    }

    pub fn get_public_url(&self, key: &str) -> String {
        format!(
            "https://{}.s3.{}.amazonaws.com/{}",
            self.bucket, self.region, key
        )
    }
}
