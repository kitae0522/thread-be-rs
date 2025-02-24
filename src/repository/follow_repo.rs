use super::RepositoryResult;
use crate::domain::model::{cursor_claims::CursorClaims, follow::FollowList};
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

#[async_trait]
pub trait FollowRepositoryTrait: Send + Sync {
    async fn follow_user(
        &self,
        user_id: i64,
        target_user_id: i64,
    ) -> RepositoryResult<bool>;

    async fn unfollow_user(
        &self,
        user_id: i64,
        target_user_id: i64,
    ) -> RepositoryResult<bool>;

    async fn is_followed_user(
        &self,
        user_id: i64,
        target_user_id: i64,
    ) -> RepositoryResult<bool>;

    async fn get_follow_status(&self, user_id: i64) -> RepositoryResult<(i64, i64)>;

    async fn list_following(
        &self,
        user_id: i64,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<FollowList>>;

    async fn list_follower(
        &self,
        user_id: i64,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<FollowList>>;

    // async fn list_recommend_user(
    //     &self,
    //     user_id: i64,
    //     limit: i64,
    // ) -> RepositoryResult<Vec<User>>;
}

pub struct FollowRepository {
    pub conn: Arc<PgPool>,
}

impl FollowRepository {
    pub fn new(conn: Arc<PgPool>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl FollowRepositoryTrait for FollowRepository {
    async fn follow_user(
        &self,
        user_id: i64,
        target_user_id: i64,
    ) -> RepositoryResult<bool> {
        let _ = sqlx::query("INSERT INTO follow (user_id, follower_id) VALUES ($1, $2)")
            .bind(user_id)
            .bind(target_user_id)
            .execute(&*self.conn)
            .await?;

        Ok(true)
    }

    async fn unfollow_user(
        &self,
        user_id: i64,
        target_user_id: i64,
    ) -> RepositoryResult<bool> {
        let _ = sqlx::query("DELETE FROM follow WHERE user_id = $1 AND follower_id = $2")
            .bind(user_id)
            .bind(target_user_id)
            .execute(&*self.conn)
            .await?;

        Ok(true)
    }

    async fn is_followed_user(
        &self,
        user_id: i64,
        target_user_id: i64,
    ) -> RepositoryResult<bool> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM follow WHERE user_id = $1 AND follower_id = $2",
        )
        .bind(user_id)
        .bind(target_user_id)
        .fetch_one(&*self.conn)
        .await?;

        Ok(count > 0)
    }

    async fn get_follow_status(&self, user_id: i64) -> RepositoryResult<(i64, i64)> {
        let (followers_count, following_count) = sqlx::query_as::<_, (i64, i64)>(
            "SELECT 
                (SELECT COUNT(*) FROM follow WHERE follower_id = $1) AS followers_count,
                (SELECT COUNT(*) FROM follow WHERE user_id = $2) AS following_count",
        )
        .bind(user_id)
        .bind(user_id)
        .fetch_one(&*self.conn)
        .await?;

        Ok((followers_count, following_count))
    }

    async fn list_following(
        &self,
        user_id: i64,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<FollowList>> {
        let following_list = sqlx::query_as::<_, FollowList>(
            r#"
            SELECT
                u.id, u.name, u.handle, u.profile_img_url, u.bio,
                f.created_at AS followed_at
            FROM follow f
            JOIN users u ON f.user_id = u.id
            WHERE f.follower_id = $1
            AND f.created_at < $2
            ORDER BY f.created_at DESC
            LIMIT $3
            "#,
        )
        .bind(user_id)
        .bind(cursor.created_at)
        .bind(limit)
        .fetch_all(&*self.conn)
        .await?;

        Ok(following_list)
    }

    async fn list_follower(
        &self,
        user_id: i64,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<FollowList>> {
        let followers = sqlx::query_as::<_, FollowList>(
            r#"
            SELECT
                u.id, u.name, u.handle, u.profile_img_url, u.bio,
                f.created_at AS followed_at
            FROM follow f
            JOIN users u ON f.follower_id = u.id
            WHERE f.user_id = $1
            AND f.created_at < $2
            ORDER BY f.created_at DESC
            LIMIT $3
            "#,
        )
        .bind(user_id)
        .bind(cursor.created_at)
        .bind(limit)
        .fetch_all(&*self.conn)
        .await?;

        Ok(followers)
    }
}
