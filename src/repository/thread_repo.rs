use async_trait::async_trait;
use chrono::Utc;
use sqlx::SqlitePool;
use std::sync::Arc;

use super::RepositoryResult;
use crate::{
    domain::{
        dto::thread::{RequestCreateThread, RequestUpdateThread},
        model::{cursor_claims::CursorClaims, thread::Thread},
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
    async fn get_thread_by_id(&self, id: i64) -> RepositoryResult<Thread>;
    async fn list_thread_by_user_id(
        &self,
        user_id: i64,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<Thread>>;
    async fn update_thread(
        &self,
        id: i64,
        new_thread: RequestUpdateThread,
    ) -> RepositoryResult<Thread>;
    async fn delete_thread(&self, id: i64) -> RepositoryResult<bool>;
    async fn list_thread_by_following(
        &self,
        user_id: i64,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<Thread>>;
    async fn list_thread_by_popularity_score(
        &self,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<Thread>>;
    async fn list_thread_by_latest_created(
        &self,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<Thread>>;
}

pub struct ThreadRepository {
    pub conn: Arc<SqlitePool>,
}

#[async_trait]
impl ThreadRepositoryTrait for ThreadRepository {
    async fn create_thread(
        &self,
        user_id: i64,
        new_thread: RequestCreateThread,
    ) -> RepositoryResult<bool> {
        let _ = sqlx::query(
            "INSERT INTO thread (user_id, title, content, parent_thread) VALUES (?, ?, ?, ?)",
        )
        .bind(user_id)
        .bind(new_thread.title)
        .bind(&new_thread.content)
        .bind(new_thread.parent_thread)
        .execute(&*self.conn)
        .await.map_err(|err| {
            println!("{:?}", err);
            CustomError::DatabaseError
        })?;

        Ok(true)
    }

    async fn get_thread_by_id(&self, id: i64) -> RepositoryResult<Thread> {
        let thread = sqlx::query_as::<_, Thread>(
            "SELECT * FROM thread WHERE id = ? AND is_deleted = FALSE",
        )
        .bind(id)
        .fetch_one(&*self.conn)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            CustomError::DatabaseError
        })?;

        Ok(thread)
    }

    async fn list_thread_by_user_id(
        &self,
        user_id: i64,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<Thread>> {
        let thread_list = sqlx::query_as::<_, Thread>(
            r#"
            SELECT
                *
            FROM thread
            WHERE is_deleted = FALSE
            AND user_id = ?
            AND created_at < ?
            AND id > ?
            ORDER BY created_at DESC, id DESC
            LIMIT ?
            "#,
        )
        .bind(user_id)
        .bind(cursor.created_at)
        .bind(cursor.id)
        .bind(limit)
        .fetch_all(&*self.conn)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            CustomError::DatabaseError
        })?;

        Ok(thread_list)
    }

    async fn update_thread(
        &self,
        id: i64,
        new_thread: RequestUpdateThread,
    ) -> RepositoryResult<Thread> {
        let affected_rows = sqlx::query(
            "UPDATE thread SET title = ?, content = ?, parent_thread = ? WHERE id = ?",
        )
        .bind(&new_thread.title)
        .bind(&new_thread.content)
        .bind(&new_thread.parent_thread)
        .bind(id)
        .execute(&*self.conn)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            CustomError::DatabaseError
        })?
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
            "UPDATE thread SET is_deleted = TRUE, deleted_at = ? WHERE id = ?",
        )
        .bind(Utc::now().to_rfc3339())
        .bind(id)
        .execute(&*self.conn)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            CustomError::DatabaseError
        })?
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
    ) -> RepositoryResult<Vec<Thread>> {
        let thread_list = sqlx::query_as::<_, Thread>(
            r#"
            SELECT
                t.*
            FROM thread t
            JOIN follow f ON f.follower_id = t.user_id
            WHERE t.is_deleted = FALSE
            AND f.user_id = ?
            AND t.created_at < ?
            ORDER BY t.created_at DESC
            LIMIT ?
            "#,
        )
        .bind(user_id)
        .bind(cursor.created_at)
        .bind(limit)
        .fetch_all(&*self.conn)
        .await
        .map_err(|_| CustomError::DatabaseError)?;

        Ok(thread_list)
    }

    async fn list_thread_by_popularity_score(
        &self,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<Thread>> {
        let thread_list = sqlx::query_as::<_, Thread>(
            r#"
            SELECT 
                t.*, 
                (COALESCE(COUNT(l.id), 0) * 2 + COALESCE(COUNT(v.id), 0) * 0.5) 
                / POWER((UNIX_TIMESTAMP(NOW()) - UNIX_TIMESTAMP(t.created_at)) / 3600 + 2, 1.5) AS adj_score
            FROM threads t
            LEFT JOIN upvote l ON l.thread_id = t.id AND l.reaction = 'UP'
            LEFT JOIN views v ON v.thread_id = t.id
            WHERE t.is_deleted = FALSE
            AND t.created_at < ?
            GROUP BY t.id, t.created_at
            ORDER BY adj_score DESC, t.created_at DESC
            LIMIT ?
            "#
        )
        .bind(cursor.created_at)
        .bind(limit)
        .fetch_all(&*self.conn)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            CustomError::DatabaseError
        })?;

        Ok(thread_list)
    }

    async fn list_thread_by_latest_created(
        &self,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<Thread>> {
        let thread_list = sqlx::query_as::<_, Thread>(
            r#"
            SELECT
                *
            FROM thread
            WHERE is_deleted = FALSE
            AND created_at < ?
            ORDER BY created_at DESC
            LIMIT ?;
            "#,
        )
        .bind(cursor.created_at)
        .bind(limit)
        .fetch_all(&*self.conn)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            CustomError::DatabaseError
        })?;

        Ok(thread_list)
    }
}
