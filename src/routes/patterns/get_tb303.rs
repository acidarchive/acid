use crate::api::models::tb303::{TB303Bar, TB303Pattern, TB303Step};
use crate::authentication::{try_extract_user_id, UserId};
use crate::configuration::CognitoSettings;
use crate::routes::patterns::PatternErrorResponse;
use crate::utils::error_chain_fmt;
use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(thiserror::Error)]
pub enum GetPatternError {
    #[error("Pattern with ID {0} not found")]
    PatternNotFound(Uuid),
    #[error("No patterns found in the database")]
    NoPatterns,
    #[error("Access denied: you don't have permission to view this pattern")]
    AccessDenied,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for GetPatternError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for GetPatternError {
    fn status_code(&self) -> StatusCode {
        match self {
            GetPatternError::PatternNotFound(_) | GetPatternError::NoPatterns => {
                StatusCode::NOT_FOUND
            }
            GetPatternError::AccessDenied => StatusCode::NOT_FOUND,
            GetPatternError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(web::Json(PatternErrorResponse {
            status: "error".to_string(),
            message: self.to_string(),
        }))
    }
}
async fn fetch_pattern_by_id(
    pool: &PgPool,
    pattern_id: Uuid,
    requesting_user_id: Option<UserId>,
) -> Result<TB303Pattern, GetPatternError> {
    let pattern = sqlx::query!(
        r#"
        SELECT
            pattern_id, user_id, name, author, title, description,
            waveform, triplets, tempo, tuning, cut_off_freq, resonance,
            env_mod, decay, accent, is_public, created_at, updated_at
        FROM patterns_tb303
        WHERE pattern_id = $1
        "#,
        pattern_id
    )
    .fetch_optional(pool)
    .await
    .context("Failed to fetch pattern details.")?
    .ok_or(GetPatternError::PatternNotFound(pattern_id))?;

    match requesting_user_id {
        Some(user_id) => {
            if !pattern.is_public.unwrap_or(false) && pattern.user_id != *user_id {
                return Err(GetPatternError::AccessDenied);
            }
        }
        None => {
            if !pattern.is_public.unwrap_or(false) {
                return Err(GetPatternError::PatternNotFound(pattern_id));
            }
        }
    }

    let rows = sqlx::query!(
        r#"
        SELECT
            b.bar_id, b.number AS bar_number,
            s.step_id AS "step_id?", s.number AS "step_number?",
            s.note, s.transpose, s.time, s.accent, s.slide
        FROM bars_tb303 b
        LEFT JOIN steps_tb303 s ON s.bar_id = b.bar_id
        WHERE b.pattern_id = $1
        ORDER BY b.number, s.number
        "#,
        pattern_id
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch bars and steps for pattern.")?;

    let mut bars: Vec<TB303Bar> = Vec::new();

    for row in rows {
        if bars.last().map(|b: &TB303Bar| b.id) != Some(row.bar_id) {
            bars.push(TB303Bar {
                id: row.bar_id,
                number: row.bar_number,
                steps: Vec::new(),
            });
        }
        if let Some(step_id) = row.step_id {
            bars.last_mut().unwrap().steps.push(TB303Step {
                id: step_id,
                number: row.step_number.unwrap(),
                note: row.note,
                transpose: row.transpose,
                time: row.time,
                accent: row.accent,
                slide: row.slide,
            });
        }
    }

    Ok(TB303Pattern {
        id: Some(pattern.pattern_id),
        name: pattern.name,
        author: pattern.author,
        title: pattern.title,
        description: pattern.description,
        tempo: pattern.tempo,
        tuning: pattern.tuning,
        waveform: pattern.waveform,
        triplets: pattern.triplets,
        cut_off_freq: pattern.cut_off_freq,
        resonance: pattern.resonance,
        env_mod: pattern.env_mod,
        decay: pattern.decay,
        accent: pattern.accent,
        created_at: Some(pattern.created_at),
        updated_at: Some(pattern.updated_at),
        is_public: pattern.is_public,
        bars,
    })
}

#[utoipa::path(
    get,
    path = "/v1/patterns/tb303/random",
    responses(
        (status = 200, description = "Random pattern retrieved successfully", body = TB303Pattern),
        (status = 404, description = "No patterns found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("token" = [])
    ),
)]
#[tracing::instrument(name = "Getting random TB303 pattern", skip(pool))]
pub async fn get_random_tb303_pattern(
    pool: web::Data<PgPool>,
) -> Result<web::Json<TB303Pattern>, GetPatternError> {
    let pattern_id = sqlx::query!(
        r#"
        SELECT pattern_id
        FROM patterns_tb303
        WHERE is_public = true
        ORDER BY RANDOM()
        LIMIT 1
        "#
    )
    .fetch_optional(pool.as_ref())
    .await
    .context("Failed to get a random pattern ID from database.")?
    .ok_or(GetPatternError::NoPatterns)?
    .pattern_id;

    let pattern = fetch_pattern_by_id(pool.as_ref(), pattern_id, None).await?;

    Ok(web::Json(pattern))
}

#[utoipa::path(
    get,
    path = "/v1/patterns/tb303/{pattern_id}",
    params(
        ("pattern_id" = String, Path, description = "The ID of the TB303 pattern to retrieve")
    ),
    responses(
        (status = 200, description = "Pattern retrieved successfully", body = TB303Pattern),
        (status = 404, description = "Pattern not found or access denied"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("token" = [])
    ),
)]
#[tracing::instrument(name = "Getting TB303 pattern by ID", skip(pool, cognito))]
pub async fn get_tb303_pattern(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    cognito: web::Data<CognitoSettings>,
    pattern_id: web::Path<Uuid>,
) -> Result<web::Json<TB303Pattern>, GetPatternError> {
    let user_id = try_extract_user_id(req.headers(), &cognito).await;
    let pattern_id = pattern_id.into_inner();

    let pattern = fetch_pattern_by_id(pool.as_ref(), pattern_id, user_id).await?;
    Ok(web::Json(pattern))
}
