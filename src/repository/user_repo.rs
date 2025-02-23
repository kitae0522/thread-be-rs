use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{debug, error};

use super::RepositoryResult;
use crate::{
    domain::{
        dto::user::{RequestSignup, RequestUpsertProfile, ResponseProfile},
        model::user::User,
    },
    error::CustomError,
    utils::crypto,
};

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn create_user(&self, new_user: RequestSignup) -> Result<String, CustomError>;
    async fn find_user_generic(
        &self,
        column: &str,
        value: &str,
    ) -> RepositoryResult<User>;
    async fn find_user_by_email(&self, email: &str) -> RepositoryResult<User>;
    async fn find_user_by_id(&self, id: i64) -> RepositoryResult<User>;
    async fn find_user_by_handle(&self, handle: &str) -> RepositoryResult<User>;
    async fn upsert_profile(
        &self,
        id: i64,
        new_profile: RequestUpsertProfile,
    ) -> RepositoryResult<ResponseProfile>;
}

pub struct UserRepository {
    pub conn: Arc<PgPool>,
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn create_user(&self, new_user: RequestSignup) -> Result<String, CustomError> {
        if self.find_user_by_email(&new_user.email).await.is_ok() {
            return Err(CustomError::AlreadyRegisteredUser(new_user.email));
        }

        let hashed_password = crypto::hash_password(&new_user.password)?;
        let result =
            sqlx::query("INSERT INTO users (email, hash_password) VALUES ($1, $2)")
                .bind(&new_user.email)
                .bind(&hashed_password)
                .execute(&*self.conn)
                .await;

        match result {
            Ok(_) => Ok("User created successfully".to_string()),
            Err(err) => {
                error!("Error creating user: {}", err);
                Err(CustomError::DatabaseError)
            }
        }
    }

    async fn find_user_by_email(&self, email: &str) -> RepositoryResult<User> {
        self.find_user_generic("email", email).await
    }

    async fn find_user_by_id(&self, id: i64) -> RepositoryResult<User> {
        self.find_user_generic("id", &id.to_string()).await
    }

    async fn find_user_by_handle(&self, handle: &str) -> RepositoryResult<User> {
        // TODO: !is_profile_complete user handling
        self.find_user_generic("handle", handle).await
    }

    async fn find_user_generic(
        &self,
        column: &str,
        value: &str,
    ) -> RepositoryResult<User> {
        let query = match column {
            "id" => "SELECT * FROM users WHERE id = $1 AND is_deleted = FALSE",
            "email" => "SELECT * FROM users WHERE email = $1 AND is_deleted = FALSE",
            "handle" => "SELECT * FROM users WHERE handle = $1 AND is_deleted = FALSE",
            _ => return Err(CustomError::InvalidQuery),
        };

        let user = sqlx::query_as::<_, User>(&query)
            .bind(value)
            .fetch_one(&*self.conn)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => CustomError::NotFound("User".to_string()),
                _ => {
                    error!("Error finding user by {}: {}", column, err);
                    CustomError::DatabaseError
                }
            })?;

        Ok(user)
    }

    async fn upsert_profile(
        &self,
        id: i64,
        new_profile: RequestUpsertProfile,
    ) -> RepositoryResult<ResponseProfile> {
        let affected_rows = sqlx::query(
            "UPDATE users SET name = $1, handle = $2, profile_img_url = $3, bio = $4, is_profile_complete = TRUE WHERE id = $5"
        )
        .bind(&new_profile.name)
        .bind(&new_profile.handle)
        .bind(&new_profile.profile_img_url)
        .bind(&new_profile.bio)
        .bind(id)
        .execute(&*self.conn)
        .await
        .map_err(|err| {
            error!("Error updating user profile: {}", err);
            CustomError::DatabaseError
        })?
        .rows_affected();

        if affected_rows > 0 {
            let profile = sqlx::query_as::<_, ResponseProfile>(
                "SELECT id, email, name, handle, profile_img_url, bio, created_at, updated_at FROM users WHERE id = $1"
            )
            .bind(id)
            .fetch_one(&*self.conn)
            .await
            .map_err(|err| {
                error!("Error fetching updated user: {}", err);
                CustomError::DatabaseError
            })?;
            Ok(profile)
        } else {
            Err(CustomError::NotFound("User".to_string()))
        }
    }
}
