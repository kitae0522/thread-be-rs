use std::sync::Arc;

use crate::{
    domain::{
        dto::thread::{
            RequestCreateThread, RequestUpdateThread, ResponseThread, ResponseThreadTree,
            ResponseThreadWithUserProfile, UserProfile,
        },
        model::cursor_claims::CursorClaims,
    },
    error::CustomError,
    repository::{
        thread_repo::ThreadRepositoryTrait, user_repo::UserRepositoryTrait,
        votes_repo::VotesRepositoryTrait,
    },
    utils,
};

pub struct ThreadService {
    user_repo: Arc<dyn UserRepositoryTrait>,
    thread_repo: Arc<dyn ThreadRepositoryTrait>,
    votes_repo: Arc<dyn VotesRepositoryTrait>,
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
        votes_repo: Arc<dyn VotesRepositoryTrait>,
    ) -> Self {
        Self { user_repo, thread_repo, votes_repo }
    }

    pub async fn create_thread(
        &self,
        user_id: i64,
        thread: RequestCreateThread,
    ) -> Result<ResponseThread, CustomError> {
        let thread_id = self.thread_repo.create_thread(user_id, thread).await?;
        let thread = self.thread_repo.get_thread_by_id(thread_id).await?;
        Ok(thread)
    }

    pub async fn get_thread_by_id(
        &self,
        id: i64,
        cursor: Option<&str>,
        limit: Option<i64>,
    ) -> Result<ResponseThreadTree, CustomError> {
        let (cursor, limit) = utils::cursor::preprocessing_cursor(cursor, limit);

        let thread = self.thread_repo.get_thread_by_id(id).await?;
        let thread = self.enrich_thread_with_user_profile(thread).await?;

        let subthread = self
            .thread_repo
            .list_subthread_by_parent_id(thread.id, cursor, limit)
            .await?;
        let subthread = self.enrich_thread_list_with_user_profile(subthread).await?;

        Ok(ResponseThreadTree { thread, subthread })
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

        let (cursor, limit) = utils::cursor::preprocessing_cursor(cursor, limit);

        let thread_list =
            self.thread_repo.list_thread_by_user_id(user.id, cursor, limit).await?;
        // let enrich_thread_list = self.enrich_thread_list_with_user_profile(thread_list).await?;
        Ok(thread_list)
    }

    pub async fn list_upvoted_thread(
        &self,
        user_id: i64,
        cursor: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<ResponseThreadWithUserProfile>, CustomError> {
        let user = self.user_repo.find_user_by_id(user_id).await?;
        if !user.is_profile_complete {
            return Err(CustomError::ProfileNotCreated);
        }

        let (cursor, limit) = utils::cursor::preprocessing_cursor(cursor, limit);

        let thread_list =
            self.votes_repo.list_upvoted_thread(user.id, cursor, limit).await?;
        let enrich_thread_list =
            self.enrich_thread_list_with_user_profile(thread_list).await?;
        Ok(enrich_thread_list)
    }

    pub async fn list_downvoted_thread(
        &self,
        user_id: i64,
        cursor: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<ResponseThreadWithUserProfile>, CustomError> {
        let user = self.user_repo.find_user_by_id(user_id).await?;
        if !user.is_profile_complete {
            return Err(CustomError::ProfileNotCreated);
        }

        let (cursor, limit) = utils::cursor::preprocessing_cursor(cursor, limit);

        let thread_list =
            self.votes_repo.list_downvoted_thread(user.id, cursor, limit).await?;
        let enrich_thread_list =
            self.enrich_thread_list_with_user_profile(thread_list).await?;
        Ok(enrich_thread_list)
    }

    pub async fn list_recommend_thread(
        &self,
        user_id: Option<i64>,
        cursor: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<ResponseThreadWithUserProfile>, CustomError> {
        let (cursor, limit) = utils::cursor::preprocessing_cursor(cursor, limit);

        let mut thread_list = match user_id {
            Some(user_id) => {
                self.list_personal_recommend_thread(user_id, cursor.clone(), limit)
                    .await?
            }
            None => self.list_guest_recommend_thread(cursor, limit).await?,
        };
        thread_list.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        let enrich_thread_list =
            self.enrich_thread_list_with_user_profile(thread_list).await?;
        Ok(enrich_thread_list.into_iter().take(limit as usize).collect())
    }

    async fn list_personal_recommend_thread(
        &self,
        user_id: i64,
        cursor_claims: CursorClaims,
        limit: i64,
    ) -> Result<Vec<ResponseThread>, CustomError> {
        let (common_threads, follower_threads) = tokio::join!(
            self.list_guest_recommend_thread(cursor_claims.clone(), limit),
            self.thread_repo.list_thread_by_following(user_id, cursor_claims, limit)
        );

        // error handling
        let common_threads = common_threads?;
        let follower_threads = follower_threads?;

        let mut personal_thread_list = Vec::new();
        personal_thread_list.extend(common_threads);
        personal_thread_list.extend(follower_threads);

        Ok(personal_thread_list)
    }

    async fn list_guest_recommend_thread(
        &self,
        cursor_claims: CursorClaims,
        limit: i64,
    ) -> Result<Vec<ResponseThread>, CustomError> {
        let (popular_threads, recent_threads) = tokio::join!(
            self.thread_repo
                .list_thread_by_popularity_score(cursor_claims.clone(), limit),
            self.thread_repo.list_thread_by_latest_created(cursor_claims, limit)
        );

        // error handling
        let popular_threads = popular_threads?;
        let recent_threads = recent_threads?;

        let mut common_thread_list = Vec::new();
        common_thread_list.extend(popular_threads);
        common_thread_list.extend(recent_threads);

        Ok(common_thread_list)
    }

    async fn check_thread_permission(
        &self,
        user_id: i64,
        thread_id: i64,
    ) -> Result<i64, CustomError> {
        let thread = self.thread_repo.get_thread_by_id(thread_id).await?;
        if user_id == thread.user_id {
            Ok(thread_id)
        } else {
            Err(CustomError::PermissionDenied(
                "You do not have permission to modify or delete this thread.".to_owned(),
            ))
        }
    }

    pub async fn update_thread_by_id(
        &self,
        user_id: i64,
        thread_id: i64,
        thread_dto: RequestUpdateThread,
    ) -> Result<ResponseThread, CustomError> {
        self.check_thread_permission(user_id, thread_id).await?;
        self.thread_repo.update_thread(thread_id, thread_dto).await
    }

    pub async fn delete_thread_by_id(
        &self,
        user_id: i64,
        thread_id: i64,
    ) -> Result<bool, CustomError> {
        self.check_thread_permission(user_id, thread_id).await?;
        self.thread_repo.delete_thread(thread_id).await
    }

    async fn enrich_thread_with_user_profile(
        &self,
        thread: ResponseThread,
    ) -> Result<ResponseThreadWithUserProfile, CustomError> {
        let user = self.user_repo.find_user_by_id(thread.user_id).await?;
        let user_profile = UserProfile {
            id: thread.user_id,
            handle: user.handle.unwrap_or_default(),
            profile_img: user.profile_img_url.unwrap_or_default(),
        };
        Ok(ResponseThreadWithUserProfile {
            id: thread.id,
            title: thread.title,
            content: thread.content,
            parent_thread: thread.parent_thread,
            votes: thread.votes,
            views: thread.views,
            reply_count: thread.reply_count,
            is_deleted: thread.is_deleted,
            deleted_at: thread.deleted_at,
            created_at: thread.created_at,
            updated_at: thread.updated_at,
            user_profile,
        })
    }

    async fn enrich_thread_list_with_user_profile(
        &self,
        thread_list: Vec<ResponseThread>,
    ) -> Result<Vec<ResponseThreadWithUserProfile>, CustomError> {
        let mut enrich_thread_list = Vec::new();
        for thread in thread_list {
            let enrich_thread = self.enrich_thread_with_user_profile(thread).await?;
            enrich_thread_list.push(enrich_thread)
        }
        Ok(enrich_thread_list)
    }
}
