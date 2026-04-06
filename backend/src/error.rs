use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use std::fmt;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("WebSocket error: {0}")]
    Ws(String),

    #[error("Authentication error: {0}")]
    Auth(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            Error::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            Error::Validation(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            Error::Auth(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            Error::Sqlx(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
            Error::Config(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error".to_string()),
            Error::Serde(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Serialization error".to_string()),
            Error::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, "IO error".to_string()),
            Error::Ws(_) => (StatusCode::INTERNAL_SERVER_ERROR, "WebSocket error".to_string()),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

impl From<sqlx::migrate::MigrateError> for Error {
    fn from(e: sqlx::migrate::MigrateError) -> Self {
        Error::Sqlx(sqlx::Error::Migrate(e))
    }
}
