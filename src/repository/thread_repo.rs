use async_trait::async_trait;

use super::RepositoryResult;
use crate::domain::{
    dto::thread::{RequestCreateThread, RequestUpdateThread},
    model::thread::Thread,
};

#[async_trait]
pub trait ThreadRepositoryTrait: Send + Sync {
    async fn create_thread(
        &self,
        new_thread: RequestCreateThread,
    ) -> RepositoryResult<Thread>;
    async fn get_thread_by_id(&self, id: i64) -> RepositoryResult<Thread>;
    async fn list_thread(
        &self,
        cursor: &str,
        limit: u64,
    ) -> RepositoryResult<Vec<Thread>>;
    async fn list_thread_by_user_id(
        &self,
        id: i64,
        curosr: &str,
        limit: u64,
    ) -> RepositoryResult<Vec<Thread>>;
    async fn list_thread_by_user_handle(
        &self,
        handle: &str,
        cursor: &str,
        limit: u64,
    ) -> RepositoryResult<Vec<Thread>>;
    async fn update_thread(
        &self,
        id: i64,
        new_thread: RequestUpdateThread,
    ) -> RepositoryResult<Thread>;
    async fn delete_thread(&self, id: i64) -> RepositoryResult<bool>;
}
