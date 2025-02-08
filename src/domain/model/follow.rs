use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct Follow {
    pub user_id: i64,
    pub follower_id: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct FollowList {
    pub id: i64,
    pub name: String,
    pub handle: String,
    pub profile_img_url: String,
    pub bio: String,
    pub followed_at: DateTime<Utc>,
}
