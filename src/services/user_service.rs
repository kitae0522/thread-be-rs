use std::sync::Arc;

use crate::{
    domain::{
        dto::user::{
            RequestSignin, RequestSignup, RequestUpsertProfile, ResponseProfile,
            ResponseSignin,
        },
        model::{
            cursor_claims::CursorClaims, follow::FollowList, jwt_claims::JwtClaims,
            user::User,
        },
    },
    error::CustomError,
    repository::{follow_repo::FollowRepositoryTrait, user_repo::UserRepositoryTrait},
    utils::{self, crypto},
};

pub struct UserService {
    user_repo: Arc<dyn UserRepositoryTrait>,
    follow_repo: Arc<dyn FollowRepositoryTrait>,
}

impl UserService {
    pub fn new(
        user_repo: Arc<dyn UserRepositoryTrait>,
        follow_repo: Arc<dyn FollowRepositoryTrait>,
    ) -> Self {
        Self { user_repo, follow_repo }
    }

    pub async fn signup(&self, user: RequestSignup) -> Result<String, CustomError> {
        if user.password != user.password_confirm {
            return Err(CustomError::PasswordMismatch);
        }

        match self.user_repo.create_user(user).await {
            Ok(msg) => Ok(msg),
            Err(msg) => Err(msg),
        }
    }

    pub async fn signin(
        &self,
        user: RequestSignin,
    ) -> Result<ResponseSignin, CustomError> {
        let user_from_db = self.user_repo.find_user_by_email(&user.email).await?;
        match crypto::verify_password(&user.password, &user_from_db.hash_password) {
            Ok(_) => {
                let token_claims = JwtClaims::new(user_from_db.id, &user_from_db.email);
                let token = JwtClaims::encode_jwt(token_claims)?;
                return Ok(ResponseSignin { token });
            }
            Err(err) => return Err(err),
        }
    }

    pub async fn me(&self, user_id: i64) -> Result<ResponseProfile, CustomError> {
        let mut user = self.user_repo.find_user_by_id(user_id).await?;
        user = self.validate_user(user).await?;

        let (follower_count, following_count) =
            self.follow_repo.get_follow_status(user.id).await?;

        Ok(ResponseProfile {
            id: user.id,
            email: user.email,
            name: user.name.unwrap_or_default(),
            handle: user.handle.unwrap_or_default(),
            profile_img_url: user.profile_img_url.unwrap_or_default(),
            bio: user.bio,
            created_at: user.created_at,
            updated_at: user.updated_at,
            follower_count,
            following_count,
        })
    }

    pub async fn upsert_profile(
        &self,
        id: i64,
        profile: RequestUpsertProfile,
    ) -> Result<ResponseProfile, CustomError> {
        let user = self.user_repo.find_user_by_id(id).await?;
        let profile = self.user_repo.upsert_profile(user.id, profile).await?;
        let (follower_count, following_count) =
            self.follow_repo.get_follow_status(user.id).await?;

        Ok(ResponseProfile {
            id: user.id,
            email: user.email,
            name: profile.name.unwrap_or_default(),
            handle: profile.handle.unwrap_or_default(),
            bio: profile.bio,
            profile_img_url: profile.profile_img_url.unwrap_or_default(),
            created_at: profile.created_at,
            updated_at: profile.updated_at,
            follower_count,
            following_count,
        })
    }

    pub async fn get_user(
        &self,
        user_handle: &str,
    ) -> Result<ResponseProfile, CustomError> {
        let mut user = self.user_repo.find_user_by_handle(user_handle).await?;
        user = self.validate_user(user).await?;

        let (follower_count, following_count) =
            self.follow_repo.get_follow_status(user.id).await?;

        Ok(ResponseProfile {
            id: user.id,
            email: user.email,
            name: user.name.unwrap_or_default(),
            handle: user.handle.unwrap_or_default(),
            profile_img_url: user.profile_img_url.unwrap_or_default(),
            bio: user.bio,
            created_at: user.created_at,
            updated_at: user.updated_at,
            follower_count,
            following_count,
        })
    }

    pub async fn list_user_follower(
        &self,
        user_handle: &str,
        cursor: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<FollowList>, CustomError> {
        let mut user = self.user_repo.find_user_by_handle(user_handle).await?;
        user = self.validate_user(user).await?;

        let cursor = cursor.unwrap_or_default();
        let claims = CursorClaims::decode_cursor(cursor).unwrap_or_default();

        let limit = limit.unwrap_or(10);
        let follower_list =
            self.follow_repo.list_follower(user.id, claims, limit).await?;
        Ok(follower_list)
    }

    pub async fn list_user_following(
        &self,
        user_handle: &str,
        cursor: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<FollowList>, CustomError> {
        let mut user = self.user_repo.find_user_by_handle(user_handle).await?;
        user = self.validate_user(user).await?;

        let (cursor, limit) = utils::cursor::preprocessing_cursor(cursor, limit);

        let following_list =
            self.follow_repo.list_following(user.id, cursor, limit).await?;
        Ok(following_list)
    }

    async fn validate_user(&self, user: User) -> Result<User, CustomError> {
        if !user.is_profile_complete {
            return Err(CustomError::ProfileNotCreated);
        }
        Ok(user)
    }
}
