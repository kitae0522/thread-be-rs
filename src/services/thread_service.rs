use std::sync::Arc;

use crate::{
    domain::{
        dto::thread::{RequestCreateThread, ResponseThread},
        model::cursor_claims::CursorClaims,
    },
    error::CustomError,
    repository::{thread_repo::ThreadRepositoryTrait, user_repo::UserRepositoryTrait},
};

pub struct ThreadService {
    user_repo: Arc<dyn UserRepositoryTrait>,
    thread_repo: Arc<dyn ThreadRepositoryTrait>,
}

// Implements cursor-based pagination.
// The `cursor` parameter is used to fetch data starting from a specific point.
// - It is a Base64-encoded string, e.g., `Base64.encode({"id":2, "created_at": "2025-02-15T06:52:51.576520123Z"})`.
// - Decoding it will extract `{ id, created_at }`.
//
// Function flow:
// 1) Retrieve the user ID (`user_id`) using `user_repo.find_user_by_handle(user_handle)`.
// 2) Decode the `cursor` value from Base64 to extract `{ id, created_at }`.
// 3) Call `thread_repo.list_thread(cursor, limit)` to fetch threads starting from the cursor.
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

    pub async fn get_thread_by_id(&self, id: i64) -> Result<ResponseThread, CustomError> {
        self.thread_repo.get_thread_by_id(id).await
    }

    pub async fn list_thread_by_user_handle(
        &self,
        user_handle: &str,
        cursor: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<ResponseThread>, CustomError> {
        let user = self.user_repo.find_user_by_handle(user_handle).await?;
        if !user.is_profile_complete {
            return Err(CustomError::ProfileNotCreated);
        }

        let cursor = cursor.unwrap_or_default();
        let claims = CursorClaims::decode_cursor(cursor).unwrap_or_default();

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
    ) -> Result<Vec<ResponseThread>, CustomError> {
        //
        let cursor = cursor.unwrap_or_default();
        let claims = CursorClaims::decode_cursor(cursor).unwrap_or_default();
        let limit = limit.unwrap_or(10);

        let (follower_threads, popular_threads, recent_threads) = tokio::join!(
            self.thread_repo.list_thread_by_following(user_id, claims.clone(), limit),
            self.thread_repo.list_thread_by_popularity_score(claims.clone(), limit),
            self.thread_repo.list_thread_by_latest_created(claims, limit)
        );

        let follower_threads = follower_threads?;
        let popular_threads = popular_threads?;
        let recent_threads = recent_threads?;

        let mut thread_list = Vec::new();
        thread_list.extend(follower_threads);
        thread_list.extend(popular_threads);
        thread_list.extend(recent_threads);

        thread_list.sort_by(|item_1, item_2| item_2.created_at.cmp(&item_1.created_at));

        Ok(thread_list.into_iter().take(limit as usize).collect())
    }
}
