use crate::api::models::tb303::TB303PatternSummary;
use crate::authentication::UserId;
use crate::common::pagination::PaginatedResponse;
use crate::routes::patterns::PatternErrorResponse;
use crate::utils::error_chain_fmt;
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;
use sqlx_paginated::{paginated_query_as, FlatQueryParams};

#[derive(thiserror::Error)]
pub enum ListPatternsError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[utoipa::path(
    get,
    path = "/v1/patterns/tb303",
    params(
        ("page" = Option<u32>, Query, description = "Page number (default: 1)"),
        ("page_size" = Option<u32>, Query, description = "Items per page (default: 10)"),
        ("sort_column" = Option<&str>, Query, description = "Column to sort by created_at, title)"),
        ("sort_direction" = Option<&str>, Query, description = "Sort direction (ascending, descending)"),
        ("search" = Option<String>, Query, description = "Search patterns by title or author"),
        ("search_columns" = Option<String>, Query, description = "Columns to search in (title, author)"),
        ("is_public" = Option<bool>, Query, description = "Filter by public/private patterns"),
    ),
    responses(
        (status = 200, description = "Pattern list retrieved successfully",
            body = PaginatedResponse<TB303PatternSummary>),
        (status = 401, description = "Unauthorized request"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("token" = [])
    ),
)]
#[tracing::instrument(name = "Listing user's TB303 patterns", skip(pool))]
pub async fn list_tb303_patterns(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
    web::Query(params): web::Query<FlatQueryParams>,
) -> Result<web::Json<PaginatedResponse<TB303PatternSummary>>, ListPatternsError> {
    let user_id = user_id.into_inner();

    let paginated_response = get_patterns(&pool, &user_id, web::Query(params))
        .await
        .context("Failed to fetch patterns")?;

    Ok(web::Json(paginated_response))
}

async fn get_patterns(
    pool: &PgPool,
    user_id: &UserId,
    web::Query(params): web::Query<FlatQueryParams>,
) -> Result<PaginatedResponse<TB303PatternSummary>, sqlx::Error> {
    let query = format!(
        "SELECT pattern_id, name, author, title, is_public, created_at, updated_at
         FROM patterns_tb303
         WHERE user_id = '{user_id}'"
    );

    let paginated_response = paginated_query_as!(TB303PatternSummary, query.as_str())
        .with_params(params)
        .fetch_paginated(pool)
        .await?;

    let paginated_response = PaginatedResponse::from(paginated_response);

    Ok(paginated_response)
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
