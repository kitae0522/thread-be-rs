use std::sync::Arc;

use crate::{
    domain::{
        dto::thread::RequestCreateThread,
        model::{cursor_claims::CursorClaims, thread::Thread},
    },
    error::CustomError,
    repository::{thread_repo::ThreadRepositoryTrait, user_repo::UserRepositoryTrait},
};

pub struct ThreadService {
    user_repo: Arc<dyn UserRepositoryTrait>,
    thread_repo: Arc<dyn ThreadRepositoryTrait>,
}

impl ThreadService {
    pub fn new(
        user_repo: Arc<dyn UserRepositoryTrait>,
        thread_repo: Arc<dyn ThreadRepositoryTrait>,
    ) -> Self {
        Self { user_repo, thread_repo }
    }

    pub async fn create_thread(
        &self,
        user_id: i64,
        thread: RequestCreateThread,
    ) -> Result<bool, CustomError> {
        self.thread_repo.create_thread(user_id, thread).await
    }

    pub async fn get_thread_by_id(&self, id: i64) -> Result<Thread, CustomError> {
        self.thread_repo.get_thread_by_id(id).await
    }

    pub async fn list_thread_by_user_handle(
        &self,
        user_handle: &str,
        cursor: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<Thread>, CustomError> {
        // Implements cursor-based pagination.
        // The `cursor` parameter is used to fetch data starting from a specific point.
        // - It is a Base64-encoded string, e.g., `Base64.encode("(id=10, user_handle=@elonmusk)")`.
        // - Decoding it will extract `{ id, user_id, created_at }`.
        //
        // Function flow:
        // 1) Retrieve the user ID (`user_id`) using `user_repo.find_user_by_handle(user_handle)`.
        // 2) Decode the `cursor` value from Base64 to extract `{ id, user_id, created_at }`.
        // 3) Call `thread_repo.list_thread(cursor, limit)` to fetch threads starting from the cursor.
        let user = self.user_repo.find_user_by_handle(user_handle).await?;
        if !user.is_profile_complete {
            return Err(CustomError::ProfileNotCreated);
        }

        let cursor = cursor.unwrap_or_default();
        let claims = CursorClaims::decode_cursor(cursor).unwrap_or_default();
        // .ok_or_else(|| {
        //     CustomError::InternalError("Failed to decode cursor".to_string())
        // })?;

        let limit = limit.unwrap_or(10);
        let thread_list =
            self.thread_repo.list_thread_by_user_id(user.id, claims, limit).await?;
        Ok(thread_list)
    }

    pub async fn list_recommend_thread(
        &self,
        user_id: i64,
        cursor: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<Thread>, CustomError> {
        let cursor = cursor.unwrap_or_default();
        let claims = CursorClaims::decode_cursor(cursor).unwrap_or_default();
        let limit = limit.unwrap_or(10);

        let follow_based_threads = self
            .thread_repo
            .list_thread_by_following(user_id, claims.clone(), limit / 2)
            .await?;
        let popularity_score_based_threads = self
            .thread_repo
            .list_thread_by_popularity_score(claims.clone(), limit / 3)
            .await?;
        let latest_created_threads =
            self.thread_repo.list_thread_by_latest_created(claims, limit / 5).await?;

        let mut thread_list = Vec::new();
        thread_list.extend(follow_based_threads);
        thread_list.extend(popularity_score_based_threads);
        thread_list.extend(latest_created_threads);

        thread_list.sort_by(|item_1, item_2| item_2.created_at.cmp(&item_1.created_at));

        Ok(thread_list)
    }
}
