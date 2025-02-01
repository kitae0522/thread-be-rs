use async_trait::async_trait;
use chrono::Utc;
use sqlx::SqlitePool;
use std::sync::Arc;

use super::RepositoryResult;
use crate::{
    domain::{
        dto::thread::{RequestCreateThread, RequestUpdateThread},
        model::thread::Thread,
    },
    error::CustomError,
};

#[async_trait]
pub trait ThreadRepositoryTrait: Send + Sync {
    async fn create_thread(
        &self,
        new_thread: RequestCreateThread,
    ) -> RepositoryResult<bool>;
    async fn get_thread_by_id(&self, id: i64) -> RepositoryResult<Thread>;
    async fn list_thread(
        &self,
        cursor: &str,
        limit: Option<i64>,
    ) -> RepositoryResult<Vec<Thread>>;
    async fn list_thread_by_user_id(
        &self,
        id: i64,
        cursor: &str,
        limit: Option<i64>,
    ) -> RepositoryResult<Vec<Thread>>;
    async fn update_thread(
        &self,
        id: i64,
        new_thread: RequestUpdateThread,
    ) -> RepositoryResult<Thread>;
    async fn delete_thread(&self, id: i64) -> RepositoryResult<bool>;
}

pub struct ThreadRepository {
    pub conn: Arc<SqlitePool>,
}

#[async_trait]
impl ThreadRepositoryTrait for ThreadRepository {
    async fn create_thread(
        &self,
        new_thread: RequestCreateThread,
    ) -> RepositoryResult<bool> {
        let title = new_thread.title.unwrap_or_default();
        let parent_thread = new_thread.parent_thread.unwrap_or_default();
        let result = sqlx::query(
            "INSERT INTO thread (title, content, parent_thread) VALUES (?, ?, ?)",
        )
        .bind(&title)
        .bind(&new_thread.content)
        .bind(parent_thread)
        .execute(&*self.conn)
        .await;

        match result {
            Ok(_) => Ok(true),
            Err(err) => {
                tracing::error!("Database error: {}", err);
                Err(CustomError::DatabaseError)
            }
        }
    }

    async fn get_thread_by_id(&self, id: i64) -> RepositoryResult<Thread> {
        let thread = sqlx::query_as::<_, Thread>(
            "SELECT * FROM thread WHERE id = ? AND is_deleted = FALSE",
        )
        .bind(id)
        .fetch_one(&*self.conn)
        .await
        .map_err(|err| {
            tracing::error!("Error finding thread by {}: {}", id, err);
            CustomError::DatabaseError
        })?;
        Ok(thread)
    }

    async fn list_thread(
        &self,
        cursor: &str,
        limit: Option<i64>,
    ) -> RepositoryResult<Vec<Thread>> {
        // TODO: Cursor based pagination
        // params:  cursor=KGlkPTEwLCB1c2VyX2lkPTEwMDEp // Base64.encode({id=10, user_id=1001})
        //          limit=10
        // query: select * from thread where id > 10 and user_id = 1001 limit 10 order by updated_at desc;
        let limit = limit.unwrap_or(10);
        let thread_list = sqlx::query_as::<_, Thread>(
            "SELECT * FROM thread WHERE is_deleted = FALSE LIMIT ?",
        )
        .bind(limit)
        .fetch_all(&*self.conn)
        .await
        .map_err(|err| {
            tracing::error!("Error finding thread: {}", err);
            CustomError::DatabaseError
        })?;
        Ok(thread_list)
    }

    async fn list_thread_by_user_id(
        &self,
        id: i64,
        cursor: &str,
        limit: Option<i64>,
    ) -> RepositoryResult<Vec<Thread>> {
        let limit = limit.unwrap_or(10);
        let thread_list = sqlx::query_as::<_, Thread>(
            "SELECT * FROM thread WHERE user_id = ? AND is_deleted = FALSE LIMIT ?",
        )
        .bind(id)
        .bind(limit)
        .fetch_all(&*self.conn)
        .await
        .map_err(|err| {
            tracing::error!("Error finding thread: {}", err);
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
            tracing::error!("Error updating thread: {}", err);
            CustomError::DatabaseError
        })?
        .rows_affected();

        if affected_rows > 0 {
            let thread = self.get_thread_by_id(id).await.map_err(|err| {
                tracing::error!("Error fetching updated thread");
                CustomError::DatabaseError
            })?;
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
            tracing::error!("Error updating thread: {}", err);
            CustomError::DatabaseError
        })?
        .rows_affected();

        if affected_rows > 0 {
            Ok(true)
        } else {
            Err(CustomError::NotFound)
        }
    }
}
