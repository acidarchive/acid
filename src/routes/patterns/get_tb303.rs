use crate::api::models::tb303::{TB303Pattern, TB303Step};
use crate::routes::patterns::PatternErrorResponse;
use crate::utils::error_chain_fmt;
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(thiserror::Error)]
pub enum GetPatternError {
    #[error("Pattern with ID {0} not found")]
    PatternNotFound(Uuid),
    #[error("No patterns found in the database")]
    NoPatterns,
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

    let steps = sqlx::query!(
        r#"
        SELECT
            step_id, pattern_id, number, note, transpose, "time", accent, slide
        FROM steps_tb303
        WHERE pattern_id = $1
        ORDER BY number
        "#,
        pattern_id
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch steps for pattern.")?;

    let steps_response: Vec<TB303Step> = steps
        .into_iter()
        .map(|step| TB303Step {
            id: step.step_id,
            number: step.number,
            note: step.note,
            transpose: step.transpose,
            time: step.time,
            accent: step.accent,
            slide: step.slide,
        })
        .collect();

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
        steps: steps_response,
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

    let pattern = fetch_pattern_by_id(pool.as_ref(), pattern_id).await?;

    Ok(web::Json(pattern))
}
