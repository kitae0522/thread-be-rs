use std::sync::Arc;

use crate::{
    domain::model::votes::ReactionType,
    error::CustomError,
    repository::{
        thread_repo::ThreadRepositoryTrait, user_repo::UserRepositoryTrait,
        votes_repo::VotesRepositoryTrait,
    },
};

pub struct VotesService {
    user_repo: Arc<dyn UserRepositoryTrait>,
    thread_repo: Arc<dyn ThreadRepositoryTrait>,
    votes_repo: Arc<dyn VotesRepositoryTrait>,
}

impl VotesService {
    pub fn new(
        user_repo: Arc<dyn UserRepositoryTrait>,
        thread_repo: Arc<dyn ThreadRepositoryTrait>,
        votes_repo: Arc<dyn VotesRepositoryTrait>,
    ) -> Self {
        Self { user_repo, thread_repo, votes_repo }
    }

    pub async fn react(
        &self,
        user_id: i64,
        target_thread_id: i64,
        reaction: ReactionType,
    ) -> Result<(), CustomError> {
        let _ = self.validate_react(user_id, target_thread_id).await?;
        if self.votes_repo.is_reacted_thread(user_id, target_thread_id).await? {
            return Err(CustomError::AlreadyReacted);
        }
        self.votes_repo.react_thread(user_id, target_thread_id, reaction).await
    }

    pub async fn react_cancel(
        &self,
        user_id: i64,
        target_thread_id: i64,
        reaction: ReactionType,
    ) -> Result<(), CustomError> {
        let _ = self.validate_react(user_id, target_thread_id).await?;
        if !self.votes_repo.is_reacted_thread(user_id, target_thread_id).await? {
            return Err(CustomError::NotReacted);
        }
        self.votes_repo.react_cancel_thread(user_id, target_thread_id, reaction).await
    }

    async fn validate_react(
        &self,
        user_id: i64,
        target_thread_id: i64,
    ) -> Result<(), CustomError> {
        let (user, thread) = tokio::join!(
            self.user_repo.find_user_by_id(user_id),
            self.thread_repo.get_thread_by_id(target_thread_id),
        );

        let user = user?;
        let thread = thread?;

        if !user.is_profile_complete {
            return Err(CustomError::ProfileNotCreated);
        }
        if thread.is_deleted {
            return Err(CustomError::NotFound);
        }

        Ok(())
    }
}
