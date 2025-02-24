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

#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct ResponseThread {
    pub id: i64,
    pub user_id: i64,

    pub title: Option<String>,
    pub content: String,
    pub parent_thread: Option<i64>,
    pub votes: i64,
    pub views: i64,
    pub reply_count: i64,

    pub is_deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseThreadTree {
    pub thread: ResponseThread,
    pub subthread: Vec<ResponseThread>,
}
