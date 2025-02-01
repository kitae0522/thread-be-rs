use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestCreateThread {
    pub title: Option<String>,
    pub content: String,
    pub parent_thread: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestUpdateThread {
    pub title: String,
    pub content: String,
    pub parent_thread: i64,
}
