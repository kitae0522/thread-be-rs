use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use crate::domain::dto::ErrorResponse;

// TODO: Controller, Service, Repository... 각 레이어 별로 에러 분리할 예정
#[derive(Debug, Serialize)]
pub enum CustomError {
    DatabaseError,
    AlreadyRegisteredUser(String),
    InvalidCredentials,
    NotFound,
    InternalError(String),
    Forbidden(String),
    Unauthorized(String),
    ProfileNotCreated,
    InvalidQuery,
    NotFollowed,
    TrySelfFollow,
    AlreadyFollowed,
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        match self {
            CustomError::DatabaseError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("Database error occurred", None)),
            )
                .into_response(),
            CustomError::AlreadyRegisteredUser(user_email) => (
                StatusCode::CONFLICT,
                Json(ErrorResponse::new(
                    &format!("User '{}' is already registerd", user_email),
                    None,
                )),
            )
                .into_response(),
            CustomError::InvalidCredentials => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new("Invalid credentials", None)),
            )
                .into_response(),
            CustomError::NotFound => {
                (StatusCode::NOT_FOUND, Json(ErrorResponse::new("Data not found", None)))
                    .into_response()
            }
            CustomError::InternalError(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(&message, None)),
            )
                .into_response(),
            CustomError::Forbidden(message) => {
                (StatusCode::FORBIDDEN, Json(ErrorResponse::new(&message, None)))
                    .into_response()
            }
            CustomError::Unauthorized(message) => {
                (StatusCode::UNAUTHORIZED, Json(ErrorResponse::new(&message, None)))
                    .into_response()
            }
            CustomError::ProfileNotCreated => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new(
                    "User profile not found. Please create your profile to continue.",
                    None,
                )),
            )
                .into_response(),
            CustomError::InvalidQuery => {
                (StatusCode::BAD_REQUEST, Json(ErrorResponse::new("Invalid Query", None)))
                    .into_response()
            }
            CustomError::NotFollowed => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new(
                    "You have not followed this user. Please check your following list.",
                    None,
                )),
            )
                .into_response(),
            CustomError::TrySelfFollow => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new(
                    "You tried to follow yourself. You cannot follow yourself.",
                    None,
                )),
            )
                .into_response(),
            CustomError::AlreadyFollowed => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new("You have already followed that user", None)),
            )
                .into_response(),
        }
    }
}
