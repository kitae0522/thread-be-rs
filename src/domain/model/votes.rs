use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct Votes {
    pub user_id: i64,
    pub thread_id: i64,
    pub reaction: ReactionType,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "reaction_enum", rename_all = "UPPERCASE")]
pub enum ReactionType {
    #[serde(rename = "UP")]
    Up,
    #[serde(rename = "DOWN")]
    Down,
}
