use crate::api::models::tb303::TB303PatternSummary;
use crate::routes::patterns::PatternErrorResponse;
use crate::utils::error_chain_fmt;
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;

#[derive(thiserror::Error)]
pub enum ListPublicPatternsError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[utoipa::path(
    get,
    path = "/v1/patterns/tb303/public",
    responses(
        (status = 200, description = "Public patterns retrieved successfully.", body = Vec<TB303PatternSummary>),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("token" = [])
    ),
)]
#[tracing::instrument(name = "Listing public TB303 patterns", skip(pool))]
pub async fn list_public_tb303_patterns(
    pool: web::Data<PgPool>,
) -> Result<web::Json<Vec<TB303PatternSummary>>, ListPublicPatternsError> {
    let patterns = fetch_public_pattern_list(&pool)
        .await
        .context("Failed to fetch public patterns")?;

    Ok(web::Json(patterns))
}

async fn fetch_public_pattern_list(pool: &PgPool) -> Result<Vec<TB303PatternSummary>, sqlx::Error> {
    let patterns = sqlx::query!(
        r#"
        SELECT pattern_id, name, author, title, is_public, created_at, updated_at
        FROM patterns_tb303
        WHERE is_public = true
        ORDER BY created_at DESC
        "#,
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

impl std::fmt::Debug for ListPublicPatternsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for ListPublicPatternsError {
    fn status_code(&self) -> StatusCode {
        match self {
            ListPublicPatternsError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
