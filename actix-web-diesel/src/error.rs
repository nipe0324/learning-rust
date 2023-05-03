use diesel::r2d2::{Error as R2D2Error, PoolError};
use thiserror::Error;
use serde_json::Value as JsonValue;

#[derive(Error, Debug)]
pub enum AppError {
    // 401
    #[error("Unauthorized: {}", _0)]
    Unauthorized(JsonValue),

    // 403
    #[error("Forbidden: {}", _0)]
    Forbidden(JsonValue),

    // 404
    #[error("Not Found: {}", _0)]
    NotFound(JsonValue),

    // 422
    #[error("Unprocessable Entity: {}", _0)]
    UnprocessableEntity(JsonValue),

    // 500
    #[error("Internal Server Error")]
    InternalServerError,
}

impl From<PoolError> for AppError {
    fn from(_: PoolError) -> Self {
        AppError::InternalServerError
    }
}
