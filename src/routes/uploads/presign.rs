use crate::api::models::uploads::{PresignRequest, PresignResponse};
use crate::authentication::UserId;
use crate::s3_client::S3Client;
use crate::utils::{get_error_response, get_fail_response};
use actix_web::{web, HttpResponse};
use std::time::Duration;
use uuid::Uuid;

const PRESIGN_EXPIRY_SECS: u64 = 300; // 5 minutes

#[utoipa::path(
    post,
    path = "/v1/uploads/presign",
    request_body = PresignRequest,
    responses(
        (status = 200, description = "Presigned URL generated successfully", body = PresignResponse),
        (status = 400, description = "Invalid upload_type or content_type"),
        (status = 401, description = "Unauthorized"),
    ),
    security(
        ("token" = [])
    )
)]
#[tracing::instrument(
    name = "Generating presigned upload URL",
    skip(s3_client, user_id, body)
)]
pub async fn presign_upload(
    s3_client: web::Data<S3Client>,
    user_id: web::ReqData<UserId>,
    body: web::Json<PresignRequest>,
) -> HttpResponse {
    let max_size = body.upload_type.max_size_bytes();
    if body.content_length > max_size {
        return HttpResponse::BadRequest().json(get_fail_response(format!(
            "File too large. Maximum size: {} bytes",
            max_size
        )));
    }

    if body.content_length == 0 {
        return HttpResponse::BadRequest()
            .json(get_fail_response("File size must be greater than 0"));
    }

    let key = format!(
        "{}/{}/{}",
        body.upload_type.s3_prefix(),
        *user_id.into_inner(),
        Uuid::new_v4()
    );

    match s3_client
        .presign_put(
            &key,
            body.content_type.as_ref(),
            body.content_length,
            Duration::from_secs(PRESIGN_EXPIRY_SECS),
        )
        .await
    {
        Ok(upload_url) => HttpResponse::Ok().json(PresignResponse { upload_url, key }),
        Err(e) => {
            tracing::error!("Failed to generate presigned URL: {}", e);
            HttpResponse::InternalServerError()
                .json(get_error_response("Failed to generate upload URL"))
        }
    }
}
