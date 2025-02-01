use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub hash_password: String,

    pub name: Option<String>,
    pub handle: Option<String>,
    pub profile_img_url: Option<String>,
    pub bio: Option<String>,

    pub is_profile_complete: bool,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
