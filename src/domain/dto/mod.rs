use chrono::Utc;
use serde::{Deserialize, Serialize};

pub mod thread;
pub mod user;

// pub type ApiResponse<T> = Result<SuccessResponse<T>, ErrorResponse>;

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    ok: bool,
    message: String,
    data: Option<T>,
    timestamp: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    ok: bool,
    message: String,
    error: Option<Vec<ErrorDetail>>,
    timestamp: String,
}

#[derive(Serialize)]
pub struct ErrorDetail {
    field: String,
    error: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestCursorParmas {
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}

impl<T> SuccessResponse<T> {
    pub fn new(message: &str, data: Option<T>) -> Self {
        Self {
            ok: true,
            message: message.to_string(),
            data,
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

impl ErrorResponse {
    pub fn new(message: &str, error: Option<Vec<ErrorDetail>>) -> Self {
        Self {
            ok: false,
            message: message.to_string(),
            error,
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}
