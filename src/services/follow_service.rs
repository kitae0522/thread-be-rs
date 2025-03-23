use std::sync::Arc;

use crate::{
    domain::model::{cursor_claims::CursorClaims, follow::FollowList, user::User},
    error::CustomError,
    repository::{follow_repo::FollowRepositoryTrait, user_repo::UserRepositoryTrait},
};

pub struct FollowService {
    user_repo: Arc<dyn UserRepositoryTrait>,
    follow_repo: Arc<dyn FollowRepositoryTrait>,
}

impl FollowService {
    pub fn new(
        user_repo: Arc<dyn UserRepositoryTrait>,
        follow_repo: Arc<dyn FollowRepositoryTrait>,
    ) -> Self {
        Self { user_repo, follow_repo }
    }

    pub async fn follow(
        &self,
        user_id: i64,
        target_user_handle: &str,
    ) -> Result<bool, CustomError> {
        // 1. When the target user does not exist
        // 2. When trying to follow yourself
        // 3. When you have already followed the user
        let target_user = self.validate_follow(user_id, target_user_handle).await?;
        if self.follow_repo.is_followed_user(user_id, target_user.id).await? {
            return Err(CustomError::AlreadyFollowed);
        }
        self.follow_repo.follow_user(user_id, target_user.id).await
    }

    pub async fn unfollow(
        &self,
        user_id: i64,
        target_user_handle: &str,
    ) -> Result<bool, CustomError> {
        // 1. When the target user does not exist
        // 2. When trying to unfollow a user that you haven't followed
        let target_user = self.validate_follow(user_id, target_user_handle).await?;
        if !self.follow_repo.is_followed_user(user_id, target_user.id).await? {
            return Err(CustomError::NotFollowed);
        }
        self.follow_repo.unfollow_user(user_id, target_user.id).await
    }

    pub async fn list_user_follower(
        &self,
        user_handle: &str,
        cursor: CursorClaims,
        limit: i64,
    ) -> Result<Vec<FollowList>, CustomError> {
        let mut user = self.user_repo.find_user_by_handle(user_handle).await?;
        user = self.validate_user(user).await?;

        let follower_list =
            self.follow_repo.list_follower(user.id, cursor, limit).await?;
        Ok(follower_list)
    }

    pub async fn list_user_following(
        &self,
        user_handle: &str,
        cursor: CursorClaims,
        limit: i64,
    ) -> Result<Vec<FollowList>, CustomError> {
        let mut user = self.user_repo.find_user_by_handle(user_handle).await?;
        user = self.validate_user(user).await?;

        let following_list =
            self.follow_repo.list_following(user.id, cursor, limit).await?;
        Ok(following_list)
    }

    async fn validate_follow(
        &self,
        user_id: i64,
        target_user_handle: &str,
    ) -> Result<User, CustomError> {
        let user = self.user_repo.find_user_by_id(user_id).await?;
        let target_user = self.user_repo.find_user_by_handle(target_user_handle).await?;
        if !user.is_profile_complete || !target_user.is_profile_complete {
            return Err(CustomError::ProfileNotCreated);
        }
        if target_user.id == user_id {
            return Err(CustomError::TrySelfFollow);
        }
        Ok(target_user)
    }

    async fn validate_user(&self, user: User) -> Result<User, CustomError> {
        if !user.is_profile_complete {
            return Err(CustomError::ProfileNotCreated);
        }
        Ok(user)
    }
}
