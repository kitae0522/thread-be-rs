use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use tracing::error;

use crate::domain::dto::ErrorResponse;

// TODO: Controller, Service, Repository... 각 레이어 별로 에러 분리할 예정
#[derive(Debug, Serialize)]
pub enum CustomError {
    DatabaseError(String),
    JWTError(String),
    AlreadyRegisteredUser(String),
    InvalidCredentials,
    NotFound,
    InternalError(String),
    PermissionDenied(String),
    Unauthorized(String),
    ProfileNotCreated,
    InvalidQuery,
    NotFollowed,
    TrySelfFollow,
    AlreadyFollowed,
    PasswordMismatch,
    AlreadyReacted,
    NotReacted,
}

impl CustomError {
    fn response_helper(&self, status_code: StatusCode, message: &str) -> Response {
        error!("status code: {}, message: {}", status_code, message);
        (status_code, Json(ErrorResponse::new(message, None))).into_response()
    }
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        match self {
            CustomError::DatabaseError(ref message) => {
                self.response_helper(StatusCode::INTERNAL_SERVER_ERROR, &message)
            }
            CustomError::JWTError(ref message) => {
                self.response_helper(StatusCode::INTERNAL_SERVER_ERROR, &message)
            }
            CustomError::AlreadyRegisteredUser(ref user_email) => self.response_helper(
                StatusCode::CONFLICT,
                &format!("User '{}' is already registerd", user_email),
            ),
            CustomError::InvalidCredentials => {
                self.response_helper(StatusCode::BAD_REQUEST, "Invalid credentials")
            }
            CustomError::NotFound => {
                self.response_helper(StatusCode::NOT_FOUND, "Data not found")
            }
            CustomError::InternalError(ref message) => {
                self.response_helper(StatusCode::INTERNAL_SERVER_ERROR, &message)
            }
            CustomError::PermissionDenied(ref message) => {
                self.response_helper(StatusCode::FORBIDDEN, &message)
            }
            CustomError::Unauthorized(ref message) => {
                self.response_helper(StatusCode::UNAUTHORIZED, &message)
            }
            CustomError::ProfileNotCreated => self.response_helper(
                StatusCode::NOT_FOUND,
                "User profile not found. Please create your profile to continue.",
            ),
            CustomError::InvalidQuery => {
                self.response_helper(StatusCode::BAD_REQUEST, "Invalid Query")
            }
            CustomError::NotFollowed => self.response_helper(
                StatusCode::BAD_REQUEST,
                "You have not followed this user. Please check your following list.",
            ),
            CustomError::TrySelfFollow => self.response_helper(
                StatusCode::BAD_REQUEST,
                "You tried to follow yourself. You cannot follow yourself.",
            ),
            CustomError::AlreadyFollowed => self.response_helper(
                StatusCode::BAD_REQUEST,
                "You have already followed that user",
            ),
            CustomError::PasswordMismatch => {
                self.response_helper(StatusCode::BAD_REQUEST, "Password not matched")
            }
            CustomError::AlreadyReacted => self.response_helper(
                StatusCode::BAD_REQUEST,
                "You have already reacted that thread",
            ),
            CustomError::NotReacted => self.response_helper(
                StatusCode::BAD_REQUEST,
                "You have not reacted this thread. Please check your reacting list.",
            ),
        }
    }
}

impl From<sqlx::Error> for CustomError {
    fn from(db_error: sqlx::Error) -> Self {
        match db_error {
            sqlx::Error::RowNotFound => CustomError::NotFound,
            _ => CustomError::DatabaseError(db_error.to_string()),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for CustomError {
    fn from(jwt_error: jsonwebtoken::errors::Error) -> Self {
        CustomError::JWTError(jwt_error.to_string())
    }
}
