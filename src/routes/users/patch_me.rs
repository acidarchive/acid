use crate::api::models::users::{UpdateUserRequest, UserResponse};
use crate::authentication::UserId;
use crate::domain::UploadType;
use crate::routes::users::UserErrorResponse;
use crate::s3_client::S3Client;
use crate::utils::error_chain_fmt;
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;

#[derive(thiserror::Error)]
pub enum PatchUserError {
    #[error("No fields to update")]
    NoFieldsToUpdate,
    #[error("Invalid key")]
    InvalidKey,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for PatchUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for PatchUserError {
    fn status_code(&self) -> StatusCode {
        match self {
            PatchUserError::NoFieldsToUpdate => StatusCode::BAD_REQUEST,
            PatchUserError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PatchUserError::InvalidKey => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(UserErrorResponse {
            status: "error".to_string(),
            message: self.to_string(),
        })
    }
}

#[utoipa::path(
    patch,
    path = "/v1/users/me",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User profile updated successfully", body = UserResponse),
        (status = 400, description = "Bad request - no fields to update"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("token" = [])
    ),
)]
#[tracing::instrument(name = "Updating current user", skip(pool, s3_client))]
pub async fn patch_me(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
    s3_client: web::Data<S3Client>,
    body: web::Json<UpdateUserRequest>,
) -> Result<web::Json<UserResponse>, PatchUserError> {
    let user_id = user_id.into_inner();

    if body.avatar_key.is_none() && body.banner_key.is_none() {
        return Err(PatchUserError::NoFieldsToUpdate);
    }

    if let Some(ref key) = body.avatar_key {
        let expected = format!("{}/{}/", UploadType::Avatar.s3_prefix(), *user_id);
        if !key.starts_with(&expected) {
            return Err(PatchUserError::InvalidKey);
        }
    }
    if let Some(ref key) = body.banner_key {
        let expected = format!("{}/{}/", UploadType::Banner.s3_prefix(), *user_id);
        if !key.starts_with(&expected) {
            return Err(PatchUserError::InvalidKey);
        }
    }

    let user = sqlx::query!(
        r#"
        INSERT INTO users (user_id, avatar_key, banner_key)
        VALUES ($1, $2, $3)
        ON CONFLICT (user_id) DO UPDATE SET
            avatar_key = COALESCE(excluded.avatar_key, users.avatar_key),
            banner_key = COALESCE(excluded.banner_key, users.banner_key),
            updated_at = NOW()
        RETURNING user_id, avatar_key, banner_key, created_at, updated_at
        "#,
        *user_id,
        body.avatar_key,
        body.banner_key
    )
    .fetch_one(pool.as_ref())
    .await
    .context("Failed to update user")?;

    let version = user.updated_at.timestamp();
    let avatar_url = user
        .avatar_key
        .as_ref()
        .map(|key| format!("{}?v={}", s3_client.get_public_url(key), version));
    let banner_url = user
        .banner_key
        .as_ref()
        .map(|key| format!("{}?v={}", s3_client.get_public_url(key), version));

    Ok(web::Json(UserResponse {
        user_id: user.user_id,
        avatar_url,
        banner_url,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }))
}
