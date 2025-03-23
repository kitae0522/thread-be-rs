use super::RepositoryResult;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

#[async_trait]
pub trait ViewsRepositoryTrait: Send + Sync {
    async fn view_thread(
        &self,
        // user_id: i64,
        target_thread_id: i64,
    ) -> RepositoryResult<()>;

    // async fn is_viewed_thread(
    //     &self,
    //     user_id: i64,
    //     target_thread_id: i64,
    // ) -> RepositoryResult<bool>;

    // async fn list_viewed_thread(
    //     &self,
    //     user_id: i64,
    //     cursor: CursorClaims,
    //     limit: i64,
    // ) -> RepositoryResult<Vec<ResponseThread>>;
}

pub struct ViewsRepository {
    pub conn: Arc<PgPool>,
}

impl ViewsRepository {
    pub fn new(conn: Arc<PgPool>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl ViewsRepositoryTrait for ViewsRepository {
    async fn view_thread(
        &self,
        // user_id: i64,
        target_thread_id: i64,
    ) -> RepositoryResult<()> {
        let _ = sqlx::query(
            r#"
            INSERT INTO views (thread_id, view_count) 
            VALUES ($1, 1)
            ON CONFLICT (thread_id) 
            DO UPDATE SET view_count = views.view_count + 1
            "#,
        )
        .bind(target_thread_id)
        .execute(&*self.conn)
        .await?;

        Ok(())
    }
}
