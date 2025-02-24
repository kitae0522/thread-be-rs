use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;

use super::RepositoryResult;
use crate::{
    domain::{
        dto::thread::{RequestCreateThread, RequestUpdateThread, ResponseThread},
        model::cursor_claims::CursorClaims,
    },
    error::CustomError,
};

#[async_trait]
pub trait ThreadRepositoryTrait: Send + Sync {
    async fn create_thread(
        &self,
        user_id: i64,
        new_thread: RequestCreateThread,
    ) -> RepositoryResult<bool>;
    async fn get_thread_by_id(&self, id: i64) -> RepositoryResult<ResponseThread>;
    async fn list_thread_by_user_id(
        &self,
        user_id: i64,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<ResponseThread>>;
    async fn update_thread(
        &self,
        id: i64,
        new_thread: RequestUpdateThread,
    ) -> RepositoryResult<ResponseThread>;
    async fn delete_thread(&self, id: i64) -> RepositoryResult<bool>;
    async fn list_thread_by_following(
        &self,
        user_id: i64,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<ResponseThread>>;
    async fn list_thread_by_popularity_score(
        &self,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<ResponseThread>>;
    async fn list_thread_by_latest_created(
        &self,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<ResponseThread>>;
    async fn list_subthread_by_parent_id(
        &self,
        thread_id: i64,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<ResponseThread>>;
}

pub struct ThreadRepository {
    pub conn: Arc<PgPool>,
}

#[async_trait]
impl ThreadRepositoryTrait for ThreadRepository {
    async fn create_thread(
        &self,
        user_id: i64,
        new_thread: RequestCreateThread,
    ) -> RepositoryResult<bool> {
        let _ = sqlx::query(
            "INSERT INTO thread (user_id, title, content, parent_thread) VALUES ($1, $2, $3, $4)",
        )
        .bind(user_id)
        .bind(new_thread.title)
        .bind(&new_thread.content)
        .bind(new_thread.parent_thread)
        .execute(&*self.conn)
        .await?;

        Ok(true)
    }

    async fn get_thread_by_id(&self, id: i64) -> RepositoryResult<ResponseThread> {
        let thread = sqlx::query_as::<_, ResponseThread>(
            r#"
            SELECT
                t.*,
                (COALESCE(SUM(CASE WHEN u.reaction = 'UP' THEN 1 ELSE 0 END), 0) + 
                COALESCE(SUM(CASE WHEN u.reaction = 'DOWN' THEN -1 ELSE 0 END), 0)) AS upvote,
                COALESCE(MAX(v.view_count), 0) AS views,
                (SELECT COUNT(*) FROM thread WHERE parent_thread = $1) AS reply_count
            FROM thread t
            LEFT JOIN upvotes u ON u.thread_id = t.id
            LEFT JOIN views v ON v.thread_id = t.id
            WHERE t.id = $1
            AND t.is_deleted = FALSE
            GROUP BY t.id
            "#,
        )
        .bind(id)
        .fetch_one(&*self.conn)
        .await?;

        Ok(thread)
    }

    async fn list_thread_by_user_id(
        &self,
        user_id: i64,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<ResponseThread>> {
        let thread_list = sqlx::query_as::<_, ResponseThread>(
            r#"
            SELECT
                t.*,
                (COALESCE(SUM(CASE WHEN u.reaction = 'UP' THEN 1 ELSE 0 END), 0) +
                COALESCE(SUM(CASE WHEN u.reaction = 'DOWN' THEN -1 ELSE 0 END), 0)) AS upvote,
                COALESCE(MAX(v.view_count), 0) AS views,
                (SELECT COUNT(*) FROM thread WHERE parent_thread = t.id) AS reply_count
            FROM thread t
            LEFT JOIN upvotes u ON u.thread_id = t.id
            LEFT JOIN views v ON v.thread_id = t.id
            WHERE t.user_id = $1
            AND t.created_at < $2
            AND t.id > $3
            AND t.is_deleted = FALSE
            GROUP BY t.id
            ORDER BY t.created_at DESC, t.id DESC
            LIMIT $4
            "#,
        )
        .bind(user_id)
        .bind(cursor.created_at)
        .bind(cursor.id)
        .bind(limit)
        .fetch_all(&*self.conn)
        .await?;

        Ok(thread_list)
    }

    async fn update_thread(
        &self,
        id: i64,
        new_thread: RequestUpdateThread,
    ) -> RepositoryResult<ResponseThread> {
        let affected_rows = sqlx::query(
            "UPDATE thread SET title = $1, content = $2, parent_thread = $3 WHERE id = $4",
        )
        .bind(&new_thread.title)
        .bind(&new_thread.content)
        .bind(&new_thread.parent_thread)
        .bind(id)
        .execute(&*self.conn)
        .await?
        .rows_affected();

        if affected_rows > 0 {
            let thread = self.get_thread_by_id(id).await?;
            Ok(thread)
        } else {
            Err(CustomError::NotFound)
        }
    }

    async fn delete_thread(&self, id: i64) -> RepositoryResult<bool> {
        let affected_rows = sqlx::query(
            "UPDATE thread SET is_deleted = TRUE, deleted_at = $1 WHERE id = $2",
        )
        .bind(Utc::now().to_rfc3339())
        .bind(id)
        .execute(&*self.conn)
        .await?
        .rows_affected();

        if affected_rows > 0 {
            Ok(true)
        } else {
            Err(CustomError::NotFound)
        }
    }

    async fn list_thread_by_following(
        &self,
        user_id: i64,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<ResponseThread>> {
        let thread_list = sqlx::query_as::<_, ResponseThread>(
            r#"
            SELECT
                t.*,
                COALESCE(SUM(CASE WHEN u.reaction = 'UP' THEN 1 ELSE 0 END), 0) +
                COALESCE(SUM(CASE WHEN u.reaction = 'DOWN' THEN -1 ELSE 0 END), 0) AS upvote,
                COALESCE(MAX(v.view_count), 0) AS views,
                (SELECT COUNT(*) FROM thread WHERE parent_thread = t.id) AS reply_count
            FROM thread t
            LEFT JOIN upvotes u ON u.thread_id = t.id
            LEFT JOIN views v ON v.thread_id = t.id
            LEFT JOIN follow f ON f.follower_id = t.user_id
            WHERE f.user_id = $1
            AND t.is_deleted = FALSE
            AND t.parent_thread IS NULL
            AND t.created_at < $2
            GROUP BY t.id
            ORDER BY t.created_at DESC
            LIMIT $3;
            "#,
        )
        .bind(user_id)
        .bind(cursor.created_at)
        .bind(limit)
        .fetch_all(&*self.conn)
        .await?;

        Ok(thread_list)
    }

    async fn list_thread_by_popularity_score(
        &self,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<ResponseThread>> {
        let thread_list = sqlx::query_as::<_, ResponseThread>(
            r#"
            SELECT 
                t.*,
                (COALESCE(SUM(CASE WHEN u.reaction = 'UP' THEN 1 ELSE 0 END), 0) +
                COALESCE(SUM(CASE WHEN u.reaction = 'DOWN' THEN -1 ELSE 0 END), 0)) AS upvote,
                COALESCE(MAX(v.view_count), 0) AS views,
                (SELECT COUNT(*) FROM thread WHERE parent_thread = t.id) AS reply_count,
                (COALESCE(COUNT(u.thread_id), 0) * 2 + COALESCE(MAX(v.view_count), 0) * 0.5) 
                / POW((EXTRACT(EPOCH FROM CURRENT_TIMESTAMP) - EXTRACT(EPOCH FROM t.created_at)) / 3600.0 + 2, 1.5) AS adj_score
            FROM thread t
            LEFT JOIN upvotes u ON u.thread_id = t.id AND u.reaction = 'UP'
            LEFT JOIN views v ON v.thread_id = t.id
            WHERE t.created_at < $1
            AND t.parent_thread IS NULL
            AND t.is_deleted = FALSE
            GROUP BY t.id, t.created_at
            ORDER BY adj_score DESC, t.created_at DESC
            LIMIT $2
            "#
        )
        .bind(cursor.created_at)
        .bind(limit)
        .fetch_all(&*self.conn)
        .await?;

        Ok(thread_list)
    }

    async fn list_thread_by_latest_created(
        &self,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<ResponseThread>> {
        let thread_list = sqlx::query_as::<_, ResponseThread>(
            r#"
            SELECT
                t.*,
                (COALESCE(SUM(CASE WHEN u.reaction = 'UP' THEN 1 ELSE 0 END), 0) +
                COALESCE(SUM(CASE WHEN u.reaction = 'DOWN' THEN -1 ELSE 0 END), 0)) AS upvote,
                COALESCE(MAX(v.view_count), 0) AS views,
                (SELECT COUNT(*) FROM thread WHERE parent_thread = t.id) AS reply_count
            FROM thread t
            LEFT JOIN upvotes u ON u.thread_id = t.id
            LEFT JOIN views v ON v.thread_id = t.id
            WHERE t.created_at < $1
            AND t.parent_thread IS NULL
            AND t.is_deleted = FALSE
            GROUP BY t.id
            ORDER BY created_at DESC
            LIMIT $2
            "#,
        )
        .bind(cursor.created_at)
        .bind(limit)
        .fetch_all(&*self.conn)
        .await?;

        Ok(thread_list)
    }

    async fn list_subthread_by_parent_id(
        &self,
        thread_id: i64,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<ResponseThread>> {
        let subthread_list = sqlx::query_as::<_, ResponseThread>(
            r#"
            SELECT
                t.*,
                (COALESCE(SUM(CASE WHEN u.reaction = 'UP' THEN 1 ELSE 0 END), 0) +
                COALESCE(SUM(CASE WHEN u.reaction = 'DOWN' THEN -1 ELSE 0 END), 0)) AS upvote,
                COALESCE(MAX(v.view_count), 0) AS views,
                (SELECT COUNT(*) FROM thread WHERE parent_thread = $1) AS reply_count
            FROM thread t
            LEFT JOIN upvotes u ON u.thread_id = t.id
            LEFT JOIN views v ON v.thread_id = t.id
            WHERE t.parent_thread = $1
            AND t.created_at < $2
            AND t.is_deleted = FALSE
            GROUP BY t.id
            ORDER BY upvote DESC, created_at DESC
            LIMIT $3
            "#,
        )
        .bind(thread_id)
        .bind(cursor.created_at)
        .bind(limit)
        .fetch_all(&*self.conn)
        .await?;

        Ok(subthread_list)
    }
}
