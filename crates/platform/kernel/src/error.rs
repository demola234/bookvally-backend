#[derive(Debug, PartialEq, thiserror::Error)]
pub enum AppError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("unprocessable entity: {0}")]
    UnprocessableEntity(String),
    #[error("internal error: {0}")]
    Internal(String),
}

impl AppError {
    pub fn internal(e: impl std::fmt::Display) -> Self {
        Self::Internal(e.to_string())
    }

    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::NotFound(resource.into())
    }
}
