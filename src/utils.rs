pub fn e500<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    status: String,
    message: String,
}

pub fn get_error_response(message: String) -> ErrorResponse {
    ErrorResponse {
        status: "error".to_string(),
        message,
    }
}

pub fn get_fail_response(message: String) -> ErrorResponse {
    ErrorResponse {
        status: "fail".to_string(),
        message,
    }
}
