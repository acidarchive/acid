use crate::api::models::tb303::CreateTB303Pattern;
use crate::authentication::UserId;
use crate::domain::{
    Author, Description, Knob, Name, NewTB303Pattern, NewTB303Step, StepNumber, Tempo, Title,
};
use crate::routes::patterns::PatternErrorResponse;
use crate::utils::error_chain_fmt;
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use anyhow::Context;
use chrono::Utc;
use sqlx::{Executor, PgPool, Postgres, QueryBuilder, Transaction};
use std::convert::TryInto;
use std::ops::Deref;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(serde::Serialize, ToSchema)]
pub struct PatternTB303Response {
    #[schema(example = "success")]
    status: String,
    data: PatternTB303ResponseData,
}

#[derive(serde::Serialize, ToSchema)]
pub struct PatternTB303ResponseData {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    id: String,
}

#[derive(thiserror::Error)]
pub enum CreatePatternError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum UpdatePatternError {
    #[error("Pattern with ID {0} not found")]
    PatternNotFound(Uuid),
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl TryInto<NewTB303Pattern> for CreateTB303Pattern {
    type Error = String;

    fn try_into(self) -> Result<NewTB303Pattern, Self::Error> {
        fn parse_optional<T, U, F>(opt: Option<U>, parse_fn: F) -> Result<Option<T>, String>
        where
            F: FnOnce(U) -> Result<T, String>,
        {
            opt.map(parse_fn).transpose().map_err(|e| e.to_string())
        }

        let name = Name::parse(self.name).map_err(|e| e.to_string())?;
        let author = parse_optional(self.author, Author::parse)?;
        let title = parse_optional(self.title, Title::parse)?;
        let description = parse_optional(self.description, Description::parse)?;
        let waveform = self.waveform;
        let triplets = self.triplets;
        let tempo = parse_optional(self.tempo, Tempo::parse)?;
        let tuning = parse_optional(self.tuning, Knob::parse)?;
        let cut_off_freq = parse_optional(self.cut_off_freq, Knob::parse)?;
        let resonance = parse_optional(self.resonance, Knob::parse)?;
        let env_mod = parse_optional(self.env_mod, Knob::parse)?;
        let decay = parse_optional(self.decay, Knob::parse)?;
        let accent = parse_optional(self.accent, Knob::parse)?;
        let is_public = self.is_public;

        if self.steps.is_empty() {
            return Err("Pattern must contain at least one step.".to_string());
        }

        let steps: Result<Vec<NewTB303Step>, String> = self
            .steps
            .into_iter()
            .map(|step| {
                let time = step.time;
                let note = step.note;
                let transpose = step.transpose;

                if time.as_ref() == "rest" && (note.is_some() || transpose.is_some()) {
                    return Err(format!(
                        "Step {} is marked as 'rest' contains a note or octave.",
                        step.number
                    ));
                }

                Ok(NewTB303Step {
                    number: StepNumber::parse(step.number).map_err(|e| e.to_string())?,
                    note,
                    transpose,
                    time,
                    accent: step.accent,
                    slide: step.slide,
                })
            })
            .collect();

        let steps = steps?;

        let mut seen_numbers = std::collections::HashSet::new();
        for step in &steps {
            if !seen_numbers.insert(step.number.as_ref()) {
                return Err(format!("Duplicate step number: {}", step.number.as_ref()));
            }
        }

        let mut step_numbers: Vec<i32> = steps.iter().map(|step| *step.number.as_ref()).collect();
        step_numbers.sort();

        if step_numbers[0] != 1 {
            return Err("Step sequence must start with 1".to_string());
        }

        for i in 1..step_numbers.len() {
            if step_numbers[i] != step_numbers[i - 1] + 1 {
                return Err(format!(
                    "Missing step in sequence: expected {}, found {}",
                    step_numbers[i - 1] + 1,
                    step_numbers[i]
                ));
            }
        }

        if steps.is_empty() {
            return Err("A pattern must have at least one step.".to_string());
        }
        if steps.len() > 16 {
            return Err("A pattern can only have up to 16 steps.".to_string());
        }

        Ok(NewTB303Pattern {
            name,
            author,
            title,
            description,
            waveform,
            triplets,
            tempo,
            tuning,
            cut_off_freq,
            resonance,
            env_mod,
            decay,
            accent,
            is_public,
            steps,
        })
    }
}

#[tracing::instrument(
    name = "Saving tb303 pattern steps in the database",
    skip(transaction, steps)
)]
pub async fn insert_steps_tb303(
    transaction: &mut Transaction<'_, Postgres>,
    pattern_id: Uuid,
    steps: &[NewTB303Step],
) -> Result<(), sqlx::Error> {
    if steps.is_empty() {
        return Ok(());
    }

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "INSERT INTO steps_tb303 (step_id, pattern_id, number, note, transpose, time, accent, slide, created_at) ",
    );

    query_builder.push_values(steps, |mut b, step| {
        let now = Utc::now();
        b.push_bind(Uuid::new_v4())
            .push_bind(pattern_id)
            .push_bind(*step.number.as_ref())
            .push_bind(step.note.as_ref().map(|n| n.as_ref()))
            .push_bind(step.transpose.as_ref().map(|s| s.as_ref()))
            .push_bind(step.time.as_ref())
            .push_bind(step.accent.unwrap_or(false))
            .push_bind(step.slide.unwrap_or(false))
            .push_bind(now);
    });

    query_builder.build().execute(&mut **transaction).await?;

    Ok(())
}

#[utoipa::path(
    request_body = CreateTB303Pattern,
    post,
    path = "/v1/patterns/tb303",
    responses(
        (status = 200, description = "Pattern created successfully", body = PatternTB303Response),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("token" = [])
    ),
)]
#[tracing::instrument(
    name = "Adding new pattern"
    skip(pattern, pool, user_id)
)]
pub async fn create_tb303_pattern(
    pattern: web::Json<CreateTB303Pattern>,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<web::Json<PatternTB303Response>, CreatePatternError> {
    let user_id = user_id.into_inner();

    let new_pattern = pattern
        .0
        .try_into()
        .map_err(CreatePatternError::ValidationError)?;

    let mut transaction = pool
        .begin()
        .await
        .context("Failed to start a new transaction.")?;

    let pattern_id = insert_pattern(&mut transaction, &new_pattern, &user_id)
        .await
        .context("Failed to insert new pattern in the database.")?;

    insert_steps_tb303(&mut transaction, pattern_id, &new_pattern.steps)
        .await
        .context("Failed to insert new pattern steps in the database.")?;

    transaction
        .commit()
        .await
        .context("Failed to commit the transaction to save tb303 pattern.")?;

    Ok(web::Json(PatternTB303Response {
        status: "success".to_string(),
        data: PatternTB303ResponseData {
            id: pattern_id.to_string(),
        },
    }))
}

#[utoipa::path(
    request_body = CreateTB303Pattern,
    put,
    path = "/v1/patterns/tb303/{pattern_id}",
    responses(
        (status = 200, description = "Pattern updated successfully", body = PatternTB303Response),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "Pattern not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("pattern_id" = String, Path, description = "ID of the pattern to update")
    ),
    security(
        ("token" = [])
    ),
)]
#[tracing::instrument(name = "Updating tb303 pattern", skip(pattern, pool, user_id))]
pub async fn update_tb303_pattern(
    pattern_id: web::Path<Uuid>,
    pattern: web::Json<CreateTB303Pattern>,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<web::Json<PatternTB303Response>, UpdatePatternError> {
    let pattern_id = pattern_id.into_inner();

    let user_id = user_id.into_inner();

    let new_pattern: NewTB303Pattern = pattern
        .0
        .try_into()
        .map_err(UpdatePatternError::ValidationError)?;

    let mut transaction = pool
        .begin()
        .await
        .context("Failed to start a transaction for update")?;

    let exists = sqlx::query_scalar!(
        r#"
        SELECT EXISTS (
            SELECT 1 FROM patterns_tb303 WHERE pattern_id = $1 AND user_id = $2
        ) AS "exists!"
        "#,
        pattern_id,
        *user_id
    )
    .fetch_one(&mut *transaction)
    .await
    .context("Failed to check if pattern exists")?;

    if !exists {
        return Err(UpdatePatternError::PatternNotFound(pattern_id));
    }

    sqlx::query!(
        r#"
        UPDATE patterns_tb303
        SET name = $1,
            author = $2,
            title = $3,
            description = $4,
            waveform = $5,
            triplets = $6,
            tempo = $7,
            tuning = $8,
            cut_off_freq = $9,
            resonance = $10,
            env_mod = $11,
            decay = $12,
            accent = $13,
            is_public = $14,
            updated_at = $15
        WHERE pattern_id = $16
        "#,
        new_pattern.name.as_ref(),
        new_pattern.author.as_ref().map(|a| a.as_ref()),
        new_pattern.title.as_ref().map(|t| t.as_ref()),
        new_pattern.description.as_ref().map(|d| d.as_ref()),
        new_pattern.waveform.as_ref().map(|w| w.as_ref()),
        new_pattern.triplets.unwrap_or(false),
        new_pattern.tempo.as_ref().map(|t| t.as_ref()),
        new_pattern
            .tuning
            .as_ref()
            .map(|v| v.as_ref())
            .unwrap_or(&0),
        new_pattern
            .cut_off_freq
            .as_ref()
            .map(|v| v.as_ref())
            .unwrap_or(&0),
        new_pattern
            .resonance
            .as_ref()
            .map(|v| v.as_ref())
            .unwrap_or(&0),
        new_pattern
            .env_mod
            .as_ref()
            .map(|v| v.as_ref())
            .unwrap_or(&0),
        new_pattern.decay.as_ref().map(|v| v.as_ref()).unwrap_or(&0),
        new_pattern
            .accent
            .as_ref()
            .map(|v| v.as_ref())
            .unwrap_or(&0),
        new_pattern.is_public.unwrap_or(false),
        Utc::now(),
        pattern_id,
    )
    .execute(&mut *transaction)
    .await
    .context("Failed to update the pattern")?;

    sqlx::query!(
        r#"DELETE FROM steps_tb303 WHERE pattern_id = $1"#,
        pattern_id
    )
    .execute(&mut *transaction)
    .await
    .context("Failed to delete old steps")?;

    insert_steps_tb303(&mut transaction, pattern_id, &new_pattern.steps)
        .await
        .context("Failed to insert updated steps")?;

    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;

    Ok(web::Json(PatternTB303Response {
        status: "success".to_string(),
        data: PatternTB303ResponseData {
            id: pattern_id.to_string(),
        },
    }))
}

#[tracing::instrument(
    name = "Saving new tb303 pattern in the database",
    skip(new_pattern, transaction, user_id)
)]
pub async fn insert_pattern(
    transaction: &mut Transaction<'_, Postgres>,
    new_pattern: &NewTB303Pattern,
    user_id: &UserId,
) -> Result<Uuid, sqlx::Error> {
    let pattern_id = Uuid::new_v4();

    let query = sqlx::query!(
        r#"
        INSERT INTO patterns_tb303 (
            pattern_id,
            user_id,
            name,
            author,
            title,
            description,
            waveform,
            triplets,
            tempo,
            tuning,
            cut_off_freq,
            resonance,
            env_mod,
            decay,
            accent,
            is_public,
            updated_at,
            created_at )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
        "#,
        pattern_id,
        user_id.deref(),
        new_pattern.name.as_ref(),
        new_pattern.author.as_ref().map(|a| a.as_ref()),
        new_pattern.title.as_ref().map(|a| a.as_ref()),
        new_pattern.description.as_ref().map(|e| e.as_ref()),
        new_pattern.waveform.as_ref().map(|w| w.as_ref()),
        new_pattern.triplets.unwrap_or(false),
        new_pattern.tempo.as_ref().map(|b| b.as_ref()),
        new_pattern
            .tuning
            .as_ref()
            .map(|t| t.as_ref())
            .unwrap_or(&0),
        new_pattern
            .cut_off_freq
            .as_ref()
            .map(|c| c.as_ref())
            .unwrap_or(&0),
        new_pattern
            .resonance
            .as_ref()
            .map(|r| r.as_ref())
            .unwrap_or(&0),
        new_pattern
            .env_mod
            .as_ref()
            .map(|e| e.as_ref())
            .unwrap_or(&0),
        new_pattern.decay.as_ref().map(|d| d.as_ref()).unwrap_or(&0),
        new_pattern
            .accent
            .as_ref()
            .map(|a| a.as_ref())
            .unwrap_or(&0),
        new_pattern.is_public.unwrap_or(false),
        Utc::now(),
        Utc::now()
    );

    transaction.execute(query).await?;

    Ok(pattern_id)
}

impl std::fmt::Debug for CreatePatternError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for CreatePatternError {
    fn status_code(&self) -> StatusCode {
        match self {
            CreatePatternError::ValidationError(_) => StatusCode::BAD_REQUEST,
            CreatePatternError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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

impl ResponseError for UpdatePatternError {
    fn status_code(&self) -> StatusCode {
        match self {
            UpdatePatternError::ValidationError(_) => StatusCode::BAD_REQUEST,
            UpdatePatternError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            UpdatePatternError::PatternNotFound(_) => StatusCode::NOT_FOUND,
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
