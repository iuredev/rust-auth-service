use argon2::password_hash::Error as Argon2Error;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::{Error as SerdeJsonError, json};
use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("Database: {0}")]
    DatabaseError(#[from] SqlxError),

    #[error("Hashing error: {0}")]
    HashingError(Argon2Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] SerdeJsonError),

    #[error("Bad request")]
    BadRequest,

    #[error("{0}")]
    Validation(String),

    #[error("Not found")]
    NotFound,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Login error: {0}")]
    LoginError(String),

    #[error("Internal server error")]
    Internal,

    #[error("Too many requests")]
    TooManyRequests,
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            MyError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            MyError::BadRequest => (StatusCode::BAD_REQUEST, self.to_string()),
            MyError::HashingError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            MyError::JsonError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            MyError::Validation(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            MyError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            MyError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            MyError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            MyError::LoginError(message) => (StatusCode::UNAUTHORIZED, message.to_string()),
            MyError::TooManyRequests => (StatusCode::TOO_MANY_REQUESTS, self.to_string()),
        };

        let body = Json(json!({
          "error": error_message
        }));

        (status, body).into_response()
    }
}
