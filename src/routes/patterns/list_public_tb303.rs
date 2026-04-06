use crate::api::models::pagination::PaginationParams;
use crate::api::models::tb303::{PaginatedTB303PatternSummary, TB303PatternSummary};
use crate::routes::patterns::PatternErrorResponse;
use crate::utils::error_chain_fmt;
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;

#[derive(thiserror::Error)]
pub enum ListPublicPatternsError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[utoipa::path(
    get,
    path = "/v1/patterns/tb303/public",
    params(PaginationParams),
    responses(
        (status = 200, description = "Public patterns retrieved successfully.", body = PaginatedTB303PatternSummary),
        (status = 400, description = "Invalid pagination parameters"),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("token" = [])
    ),
)]
#[tracing::instrument(name = "Listing public TB303 patterns", skip(pool))]
pub async fn list_public_tb303_patterns(
    pool: web::Data<PgPool>,
    query: web::Query<PaginationParams>,
) -> Result<web::Json<PaginatedTB303PatternSummary>, ListPublicPatternsError> {
    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);

    if !(1..=100).contains(&limit) {
        return Err(ListPublicPatternsError::ValidationError(
            "limit must be between 1 and 100".to_string(),
        ));
    }
    if offset < 0 {
        return Err(ListPublicPatternsError::ValidationError(
            "offset must be 0 or greater".to_string(),
        ));
    }

    let response = fetch_public_pattern_list(&pool, limit, offset)
        .await
        .context("Failed to fetch public patterns")?;

    Ok(web::Json(response))
}

async fn fetch_public_pattern_list(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<PaginatedTB303PatternSummary, sqlx::Error> {
    let total: i64 =
        sqlx::query_scalar!("SELECT COUNT(*) FROM patterns_tb303 WHERE is_public = true")
            .fetch_one(pool)
            .await?
            .unwrap_or(0);

    let patterns = sqlx::query!(
        r#"
        SELECT
            pattern_id, name, author, title, is_public, created_at, updated_at
        FROM patterns_tb303
        WHERE is_public = true
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;

    let data: Vec<TB303PatternSummary> = patterns
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

    Ok(PaginatedTB303PatternSummary {
        data,
        total,
        limit,
        offset,
    })
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
            ListPublicPatternsError::ValidationError(_) => StatusCode::BAD_REQUEST,
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
