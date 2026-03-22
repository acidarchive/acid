use crate::api::models::users::UserResponse;
use crate::authentication::UserId;
use crate::routes::users::UserErrorResponse;
use crate::s3_client::S3Client;
use crate::utils::error_chain_fmt;
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;

#[derive(thiserror::Error)]
pub enum GetUserError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for GetUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for GetUserError {
    fn status_code(&self) -> StatusCode {
        match self {
            GetUserError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
    get,
    path = "/v1/users/me",
    responses(
        (status = 200, description = "User profile retrieved successfully", body = UserResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("token" = [])
    ),
)]
#[tracing::instrument(name = "Getting current user", skip(pool, s3_client))]
pub async fn get_me(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
    s3_client: web::Data<S3Client>,
) -> Result<web::Json<UserResponse>, GetUserError> {
    let user_id = user_id.into_inner();

    let user = sqlx::query!(
        r#"
        SELECT user_id, username, avatar_key, banner_key, created_at, updated_at
        FROM users
        WHERE user_id = $1
        "#,
        *user_id
    )
    .fetch_one(pool.as_ref())
    .await
    .context("Failed to fetch user")?;

    let avatar_url = user
        .avatar_key
        .as_ref()
        .map(|key| s3_client.get_public_url(key));

    let banner_url = user
        .banner_key
        .as_ref()
        .map(|key| s3_client.get_public_url(key));

    Ok(web::Json(UserResponse {
        user_id: user.user_id,
        username: user.username,
        avatar_url,
        banner_url,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }))
}
