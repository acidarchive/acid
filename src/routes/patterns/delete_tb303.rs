use crate::authentication::UserId;
use crate::utils::error_chain_fmt;
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(thiserror::Error)]
pub enum DeletePatternError {
    #[error("Pattern with ID {0} not found")]
    PatternNotFound(Uuid),
    #[error("Access denied: you don't have permission to delete this pattern")]
    AccessDenied,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for DeletePatternError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for DeletePatternError {
    fn status_code(&self) -> StatusCode {
        match self {
            DeletePatternError::PatternNotFound(_) => StatusCode::NOT_FOUND,
            DeletePatternError::AccessDenied => StatusCode::FORBIDDEN,
            DeletePatternError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[utoipa::path(
    delete,
    path = "/v1/patterns/tb303/{pattern_id}",
    params(
        ("pattern_id" = String, Path, description = "The ID of the TB303 pattern to delete")
    ),
    responses(
        (status = 204, description = "Pattern deleted successfully"),
        (status = 403, description = "Access denied"),
        (status = 404, description = "Pattern not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("token" = [])
    ),
)]
#[tracing::instrument(name = "Deleting TB303 pattern by ID", skip(pool, user_id))]
pub async fn delete_tb303_pattern(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
    pattern_id: web::Path<Uuid>,
) -> Result<HttpResponse, DeletePatternError> {
    let pattern_id = pattern_id.into_inner();
    let user_id = user_id.into_inner();

    delete_pattern_by_id(pool.as_ref(), pattern_id, *user_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

async fn delete_pattern_by_id(
    pool: &PgPool,
    pattern_id: Uuid,
    requesting_user_id: Uuid,
) -> Result<(), DeletePatternError> {
    let pattern_user_id = sqlx::query!(
        r#"
        SELECT user_id FROM patterns_tb303 WHERE pattern_id = $1
        "#,
        pattern_id
    )
    .fetch_optional(pool)
    .await
    .context("Failed to fetch pattern owner.")?
    .map(|row| row.user_id);

    match pattern_user_id {
        Some(owner_id) => {
            if owner_id != requesting_user_id {
                return Err(DeletePatternError::AccessDenied);
            }
        }
        None => return Err(DeletePatternError::PatternNotFound(pattern_id)),
    }

    let mut transaction = pool
        .begin()
        .await
        .context("Failed to begin a database transaction.")?;

    sqlx::query!(
        r#"
        DELETE FROM steps_tb303 WHERE pattern_id = $1
        "#,
        pattern_id
    )
    .execute(&mut *transaction)
    .await
    .context("Failed to delete steps for pattern.")?;

    let result = sqlx::query!(
        r#"
        DELETE FROM patterns_tb303 WHERE pattern_id = $1 AND user_id = $2
        "#,
        pattern_id,
        requesting_user_id
    )
    .execute(&mut *transaction)
    .await
    .context("Failed to delete pattern.")?;

    if result.rows_affected() == 0 {
        // This case should ideally not be reached if the initial check passes,
        // but it's a safeguard.
        return Err(DeletePatternError::PatternNotFound(pattern_id));
    }

    transaction
        .commit()
        .await
        .context("Failed to commit the database transaction.")?;

    Ok(())
}
