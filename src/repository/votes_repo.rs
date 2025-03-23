use super::RepositoryResult;
use crate::domain::{
    dto::thread::ResponseThread,
    model::{cursor_claims::CursorClaims, votes::ReactionType},
};
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

#[async_trait]
pub trait VotesRepositoryTrait: Send + Sync {
    async fn react_thread(
        &self,
        user_id: i64,
        target_thread_id: i64,
        reaction_type: ReactionType,
    ) -> RepositoryResult<()>;

    async fn react_cancel_thread(
        &self,
        user_id: i64,
        target_thread_id: i64,
        reaction: ReactionType,
    ) -> RepositoryResult<()>;

    async fn is_reacted_thread(
        &self,
        user_id: i64,
        target_thread_id: i64,
    ) -> RepositoryResult<bool>;

    async fn list_upvoted_thread(
        &self,
        user_id: i64,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<ResponseThread>>;

    async fn list_downvoted_thread(
        &self,
        user_id: i64,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<ResponseThread>>;
}

pub struct VotesRepository {
    pub conn: Arc<PgPool>,
}

impl VotesRepository {
    pub fn new(conn: Arc<PgPool>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl VotesRepositoryTrait for VotesRepository {
    async fn react_thread(
        &self,
        user_id: i64,
        target_thread_id: i64,
        reaction: ReactionType,
    ) -> RepositoryResult<()> {
        // let reaction = match reaction_type {
        //     ReactionType::Up => "UP",
        //     ReactionType::Down => "DOWN",
        // };

        let _ = sqlx::query(
            "INSERT INTO votes (user_id, thread_id, reaction) VALUES ($1, $2, $3)",
        )
        .bind(user_id)
        .bind(target_thread_id)
        .bind(reaction)
        .execute(&*self.conn)
        .await?;

        Ok(())
    }

    async fn react_cancel_thread(
        &self,
        user_id: i64,
        target_thread_id: i64,
        reaction: ReactionType,
    ) -> RepositoryResult<()> {
        let _ = sqlx::query(
            "DELETE FROM votes WHERE user_id = $1 AND thread_id = $2 AND reaction = $3",
        )
        .bind(user_id)
        .bind(target_thread_id)
        .bind(reaction)
        .execute(&*self.conn)
        .await?;
        Ok(())
    }

    async fn is_reacted_thread(
        &self,
        user_id: i64,
        target_thread_id: i64,
    ) -> RepositoryResult<bool> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM votes WHERE user_id = $1 AND thread_id = $2",
        )
        .bind(user_id)
        .bind(target_thread_id)
        .fetch_one(&*self.conn)
        .await?;

        Ok(count > 0)
    }

    async fn list_upvoted_thread(
        &self,
        user_id: i64,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<ResponseThread>> {
        let upvoted_list = sqlx::query_as::<_, ResponseThread>(
            r#"
            SELECT
                t.*,
                (COALESCE(SUM(CASE WHEN vts.reaction = 'UP' THEN 1 ELSE 0 END), 0) +
                COALESCE(SUM(CASE WHEN vts.reaction = 'DOWN' THEN -1 ELSE 0 END), 0)) AS votes,
                COALESCE(MAX(v.view_count), 0) AS views,
                (SELECT COUNT(*) FROM thread WHERE parent_thread = t.id) AS reply_count,
                MAX(u.created_at) AS reacted_at
            FROM thread t
            JOIN votes u 
                ON u.thread_id = t.id 
                AND u.user_id = $1 
                AND u.reaction = 'UP'
            LEFT JOIN votes vts 
                ON vts.thread_id = t.id
            LEFT JOIN views v 
                ON v.thread_id = t.id
            WHERE t.is_deleted = FALSE
            AND t.created_at < $2
            GROUP BY t.id
            ORDER BY reacted_at DESC, t.id DESC
            LIMIT $3
            "#
        )
        .bind(user_id)
        .bind(cursor.created_at)
        .bind(limit)
        .fetch_all(&*self.conn)
        .await?;

        Ok(upvoted_list)
    }

    async fn list_downvoted_thread(
        &self,
        user_id: i64,
        cursor: CursorClaims,
        limit: i64,
    ) -> RepositoryResult<Vec<ResponseThread>> {
        let downvoted_list = sqlx::query_as::<_, ResponseThread>(
            r#"
            SELECT
                t.*,
                (COALESCE(SUM(CASE WHEN vts.reaction = 'UP' THEN 1 ELSE 0 END), 0) +
                COALESCE(SUM(CASE WHEN vts.reaction = 'DOWN' THEN -1 ELSE 0 END), 0)) AS votes,
                COALESCE(MAX(v.view_count), 0) AS views,
                (SELECT COUNT(*) FROM thread WHERE parent_thread = t.id) AS reply_count,
                MAX(u.created_at) AS reacted_at
            FROM thread t
            JOIN votes u 
                ON u.thread_id = t.id 
                AND u.user_id = $1 
                AND u.reaction = 'DOWN'
            LEFT JOIN votes vts 
                ON vts.thread_id = t.id
            LEFT JOIN views v 
                ON v.thread_id = t.id
            WHERE t.is_deleted = FALSE
            AND t.created_at < $2
            GROUP BY t.id
            ORDER BY reacted_at DESC, t.id DESC
            LIMIT $3
            "#
        )
        .bind(user_id)
        .bind(cursor.created_at)
        .bind(limit)
        .fetch_all(&*self.conn)
        .await?;

        Ok(downvoted_list)
    }
}
