use actix_web::{HttpResponse, ResponseError};
use sqlx::error::Error as SqlxError;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DbError(SqlxError),
    NotFound(String),
    InvalidInput(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::DbError(err) => write!(f, "Database error: {}", err),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DbError(_) => {
                HttpResponse::InternalServerError().json("Internal server error")
            }
            AppError::NotFound(msg) => HttpResponse::NotFound().json(msg),
            AppError::InvalidInput(msg) => HttpResponse::BadRequest().json(msg),
        }
    }
}

impl From<SqlxError> for AppError {
    fn from(err: SqlxError) -> Self {
        AppError::DbError(err)
    }
}
