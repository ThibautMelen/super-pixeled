//! Request validation utilities.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

/// API error response.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

/// API error types.
#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    Validation(String),
    NotFound(String),
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error, message) = match self {
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg),
            Self::Validation(msg) => (StatusCode::BAD_REQUEST, "validation_error", msg),
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg),
            Self::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", msg),
        };

        let body = Json(ErrorResponse {
            error: error.to_string(),
            message,
        });

        (status, body).into_response()
    }
}

impl From<sp_core::Error> for ApiError {
    fn from(err: sp_core::Error) -> Self {
        match err {
            sp_core::Error::EffectNotFound(name) => Self::NotFound(format!("Effect not found: {name}")),
            sp_core::Error::InvalidParameter { field, message } => {
                Self::Validation(format!("{field}: {message}"))
            }
            _ => Self::Internal(err.to_string()),
        }
    }
}
