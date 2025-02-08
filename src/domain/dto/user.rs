use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestSignup {
    pub email: String,
    pub password: String,
    pub password_confirm: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestSignin {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseSignin {
    pub token: String,
}

#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct ResponseProfile {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub handle: String,
    pub profile_img_url: String,
    pub bio: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub follower_count: u64,
    pub following_count: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestUpsertProfile {
    pub name: String,
    pub handle: String,
    pub profile_img_url: String,
    pub bio: String,
}
