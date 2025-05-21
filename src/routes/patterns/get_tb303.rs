use crate::routes::patterns::PatternErrorResponse;
use crate::utils::error_chain_fmt;
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use anyhow::Context;
use serde::Serialize;
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct TB303PatternResponse {
    #[schema(example = "success")]
    status: String,
    data: TB303PatternData,
}

#[derive(Serialize, ToSchema)]
pub struct TB303PatternData {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    id: String,
    #[schema(example = "user123")]
    author: Option<String>,
    #[schema(example = "My cool pattern")]
    title: Option<String>,
    #[schema(example = "This is a pattern")]
    description: Option<String>,
    #[schema(example = 120)]
    bpm: Option<i32>,
    #[schema(example = "sawtooth")]
    waveform: Option<String>,
    #[schema(example = false)]
    triplets: Option<bool>,
    #[schema(example = 50)]
    cut_off_freq: Option<i32>,
    #[schema(example = 50)]
    resonance: Option<i32>,
    #[schema(example = 50)]
    env_mod: Option<i32>,
    #[schema(example = 50)]
    decay: Option<i32>,
    #[schema(example = 50)]
    accent: Option<i32>,
    steps: Vec<TB303StepData>,
}

#[derive(Serialize, ToSchema)]
pub struct TB303StepData {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    id: String,
    #[schema(example = 1)]
    number: i32,
    #[schema(example = "C")]
    note: Option<String>,
    #[schema(example = "up")]
    octave: Option<String>,
    #[schema(example = "note")]
    time: Option<String>,
    #[schema(example = true)]
    accent: Option<bool>,
    #[schema(example = false)]
    slide: Option<bool>,
}

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
) -> Result<TB303PatternData, GetPatternError> {
    let pattern = sqlx::query!(
        r#"
        SELECT
            pattern_id, user_id, author, title, description,
            waveform, triplets, bpm, cut_off_freq, resonance,
            env_mod, decay, accent, created_at, updated_at
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
            step_id, pattern_id, number, note, octave, "time", accent, slide
        FROM steps_tb303
        WHERE pattern_id = $1
        ORDER BY number
        "#,
        pattern_id
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch steps for pattern.")?;

    let steps_response: Vec<TB303StepData> = steps
        .into_iter()
        .map(|step| TB303StepData {
            id: step.step_id.to_string(),
            number: step.number,
            note: step.note,
            octave: step.octave,
            time: step.time,
            accent: step.accent,
            slide: step.slide,
        })
        .collect();

    Ok(TB303PatternData {
        id: pattern.pattern_id.to_string(),
        author: pattern.author,
        title: pattern.title,
        description: pattern.description,
        bpm: pattern.bpm,
        waveform: pattern.waveform,
        triplets: pattern.triplets,
        cut_off_freq: pattern.cut_off_freq,
        resonance: pattern.resonance,
        env_mod: pattern.env_mod,
        decay: pattern.decay,
        accent: pattern.accent,
        steps: steps_response,
    })
}

#[utoipa::path(
    get,
    path = "/v1/patterns/tb303/random",
    responses(
        (status = 200, description = "Random pattern retrieved successfully", body = TB303PatternResponse),
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
) -> Result<web::Json<TB303PatternResponse>, GetPatternError> {
    let pattern_id = sqlx::query!(
        r#"
        SELECT pattern_id
        FROM patterns_tb303
        ORDER BY RANDOM()
        LIMIT 1
        "#
    )
    .fetch_optional(pool.as_ref())
    .await
    .context("Failed to get a random pattern ID from database.")?
    .ok_or(GetPatternError::NoPatterns)?
    .pattern_id;

    let pattern_data = fetch_pattern_by_id(pool.as_ref(), pattern_id).await?;

    Ok(web::Json(TB303PatternResponse {
        status: "success".to_string(),
        data: pattern_data,
    }))
}
