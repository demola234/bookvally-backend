use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use kernel::AppError;
use serde_json::json;

pub struct HttpError(pub AppError);

impl From<AppError> for HttpError {
    fn from(e: AppError) -> Self { Self(e) }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let e = self.0;
        let status = match &e {
            AppError::NotFound(_)            => StatusCode::NOT_FOUND,
            AppError::Unauthorized           => StatusCode::UNAUTHORIZED,
            AppError::Forbidden              => StatusCode::FORBIDDEN,
            AppError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::Internal(_)            => StatusCode::INTERNAL_SERVER_ERROR,
        };

        // Never leak internal details to clients
        let message = match &e {
            AppError::Internal(_) => "internal server error".to_string(),
            other => other.to_string(),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
