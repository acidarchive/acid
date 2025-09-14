use crate::api::models::tb303::TB303PatternSummary;
use crate::authentication::UserId;
use crate::routes::patterns::PatternErrorResponse;
use crate::utils::error_chain_fmt;
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;
use std::ops::Deref;

#[derive(thiserror::Error)]
pub enum ListPatternsError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[utoipa::path(
    get,
    path = "/v1/patterns/tb303",
    responses(
        (status = 200, description = "Pattern list retrieved successfully.", body = Vec<TB303PatternSummary>),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("token" = [])
    ),

)]
#[tracing::instrument(name = "Listing user's TB303 patterns", skip(pool, user_id))]
pub async fn list_tb303_patterns(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<web::Json<Vec<TB303PatternSummary>>, ListPatternsError> {
    let user_id = user_id.into_inner();

    let patterns = fetch_pattern_list(&pool, &user_id)
        .await
        .context("Failed to fetch patterns")?;

    Ok(web::Json(patterns))
}

async fn fetch_pattern_list(
    pool: &PgPool,
    user_id: &UserId,
) -> Result<Vec<TB303PatternSummary>, sqlx::Error> {
    let patterns = sqlx::query!(
        r#"
        SELECT pattern_id, name, author, title, is_public, created_at, updated_at
        FROM patterns_tb303
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
        user_id.deref()
    )
    .fetch_all(pool)
    .await?;

    let patterns_response: Vec<TB303PatternSummary> = patterns
        .into_iter()
        .map(|pattern| TB303PatternSummary {
            pattern_id: pattern.pattern_id,
            name: pattern.name,
            author: pattern.author,
            title: pattern.title,
            is_public: pattern.is_public.unwrap(),
            created_at: pattern.created_at,
            updated_at: pattern.updated_at,
        })
        .collect();

    Ok(patterns_response)
}

impl std::fmt::Debug for ListPatternsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for ListPatternsError {
    fn status_code(&self) -> StatusCode {
        match self {
            ListPatternsError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error = PatternErrorResponse {
            status: "error".to_string(),
            message: self.to_string(),
        };
        HttpResponse::build(self.status_code()).json(web::Json(error))
    }
}
