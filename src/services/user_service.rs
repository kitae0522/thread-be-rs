use std::sync::Arc;

use crate::{
    domain::{
        dto::user::{
            RequestSignin, RequestSignup, RequestUpsertProfile, ResponseProfile,
            ResponseSignin,
        },
        model::jwt_claims::JwtClaims,
    },
    error::CustomError,
    repository::user_repo::UserRepositoryTrait,
    utils::crypto,
};

pub struct UserService {
    user_repo: Arc<dyn UserRepositoryTrait>,
}

impl UserService {
    pub fn new(user_repo: Arc<dyn UserRepositoryTrait>) -> Self {
        Self { user_repo }
    }

    pub async fn signup(&self, user: RequestSignup) -> Result<String, CustomError> {
        match self.user_repo.create_user(user).await {
            Ok(msg) => Ok(msg),
            Err(msg) => Err(msg),
        }
    }

    pub async fn signin(
        &self,
        user: RequestSignin,
    ) -> Result<ResponseSignin, CustomError> {
        if let Ok(user_from_db) = self.user_repo.find_user_by_email(&user.email).await {
            match crypto::verify_password(&user.password, &user_from_db.hash_password) {
                Ok(_) => {
                    let token = JwtClaims::new(user_from_db.id, &user_from_db.email);
                    return Ok(ResponseSignin { token });
                }
                Err(err) => return Err(err),
            }
        }
        Err(CustomError::DatabaseError)
    }

    pub async fn me(&self, user_id: i64) -> Result<ResponseProfile, CustomError> {
        if let Ok(user_from_db) = self.user_repo.find_user_by_id(user_id).await {
            if user_from_db.is_profile_complete {
                return Ok(ResponseProfile {
                    id: user_from_db.id,
                    email: user_from_db.email,
                    name: user_from_db.name.unwrap_or_default(),
                    handle: user_from_db.handle.unwrap_or_default(),
                    profile_img_url: user_from_db.profile_img_url.unwrap_or_default(),
                    bio: user_from_db.bio,
                    created_at: user_from_db.created_at,
                    updated_at: user_from_db.updated_at,
                });
            } else {
                return Err(CustomError::ProfileNotCreated);
            }
        }
        Err(CustomError::DatabaseError)
    }

    pub async fn upsert_profile(
        &self,
        id: i64,
        profile: RequestUpsertProfile,
    ) -> Result<ResponseProfile, CustomError> {
        if let Ok(user_from_db) = self.user_repo.find_user_by_id(id).await {
            if let Ok(profile) =
                self.user_repo.upsert_profile(user_from_db.id, profile).await
            {
                return Ok(profile);
            }
            return Err(CustomError::DatabaseError);
        }
        Err(CustomError::DatabaseError)
    }

    pub async fn get_user(
        &self,
        user_handle: &str,
    ) -> Result<ResponseProfile, CustomError> {
        if let Ok(user_from_db) = self.user_repo.find_user_by_handle(user_handle).await {
            if user_from_db.is_profile_complete {
                return Ok(ResponseProfile {
                    id: user_from_db.id,
                    email: user_from_db.email,
                    name: user_from_db.name.unwrap_or_default(),
                    handle: user_from_db.handle.unwrap_or_default(),
                    profile_img_url: user_from_db.profile_img_url.unwrap_or_default(),
                    bio: user_from_db.bio,
                    created_at: user_from_db.created_at,
                    updated_at: user_from_db.updated_at,
                });
            } else {
                return Err(CustomError::ProfileNotCreated);
            }
        }
        Err(CustomError::DatabaseError)
    }
}
