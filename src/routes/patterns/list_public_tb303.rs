use crate::api::models::pagination::PaginationParams;
use crate::api::models::sort::SortParams;
use crate::api::models::tb303::{PaginatedTB303PatternSummary, TB303PatternSummary};
use crate::routes::patterns::PatternErrorResponse;
use crate::utils::error_chain_fmt;
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use anyhow::Context;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

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
    pagination: web::Query<PaginationParams>,
    sort: web::Query<SortParams>,
) -> Result<web::Json<PaginatedTB303PatternSummary>, ListPublicPatternsError> {
    let limit = pagination.limit.unwrap_or(20);
    let offset = pagination.offset.unwrap_or(0);
    let order = sort.order.as_deref().unwrap_or("desc").to_lowercase();

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
    if order != "asc" && order != "desc" {
        return Err(ListPublicPatternsError::ValidationError(
            "order must be \"asc\" or \"desc\"".to_string(),
        ));
    }

    let response = fetch_public_pattern_list(&pool, limit, offset, &order)
        .await
        .context("Failed to fetch public patterns")?;

    Ok(web::Json(response))
}

#[derive(sqlx::FromRow)]
struct PublicPatternRow {
    pattern_id: Uuid,
    name: String,
    author: Option<String>,
    title: Option<String>,
    is_public: Option<bool>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

async fn fetch_public_pattern_list(
    pool: &PgPool,
    limit: i64,
    offset: i64,
    order: &str,
) -> Result<PaginatedTB303PatternSummary, sqlx::Error> {
    let total: i64 =
        sqlx::query_scalar!("SELECT COUNT(*) FROM patterns_tb303 WHERE is_public = true")
            .fetch_one(pool)
            .await?
            .unwrap_or(0);

    let mut builder = sqlx::QueryBuilder::new(
        r#"SELECT pattern_id, name, author, title, is_public, created_at, updated_at
                 FROM patterns_tb303
                 WHERE is_public = true"#,
    );

    builder.push(" ORDER BY created_at ");
    builder.push(if order == "asc" { "ASC" } else { "DESC" });
    builder.push(" LIMIT ").push_bind(limit);
    builder.push(" OFFSET ").push_bind(offset);

    let rows = builder
        .build_query_as::<PublicPatternRow>()
        .fetch_all(pool)
        .await?;

    let data = rows
        .into_iter()
        .map(|r| TB303PatternSummary {
            pattern_id: r.pattern_id,
            name: r.name,
            author: r.author,
            title: r.title,
            is_public: r.is_public.unwrap(),
            created_at: r.created_at,
            updated_at: r.updated_at,
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
