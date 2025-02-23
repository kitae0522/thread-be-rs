use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

use super::RepositoryResult;
use crate::{
    domain::{
        dto::user::{RequestSignup, RequestUpsertProfile},
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
    ) -> RepositoryResult<User>;
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
        let _ = sqlx::query("INSERT INTO users (email, hash_password) VALUES ($1, $2)")
            .bind(&new_user.email)
            .bind(&hashed_password)
            .execute(&*self.conn)
            .await?;

        Ok("User created successfully".to_string())
    }

    async fn find_user_by_email(&self, email: &str) -> RepositoryResult<User> {
        self.find_user_generic("email", email).await
    }

    async fn find_user_by_id(&self, id: i64) -> RepositoryResult<User> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1 AND is_deleted = FALSE",
        )
        .bind(id)
        .fetch_one(&*self.conn)
        .await?;
        Ok(user)
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
            "email" => "SELECT * FROM users WHERE email = $1 AND is_deleted = FALSE",
            "handle" => "SELECT * FROM users WHERE handle = $1 AND is_deleted = FALSE",
            _ => return Err(CustomError::InvalidQuery),
        };

        let user =
            sqlx::query_as::<_, User>(&query).bind(value).fetch_one(&*self.conn).await?;
        Ok(user)
    }

    async fn upsert_profile(
        &self,
        id: i64,
        new_profile: RequestUpsertProfile,
    ) -> RepositoryResult<User> {
        let updated_user = sqlx::query_as::<_, User>(
            r#"
            UPDATE
                users
            SET
                name = $1,
                handle = $2,
                profile_img_url = $3,
                bio = $4,
                is_profile_complete = TRUE
            WHERE
                id = $5
            RETURNING
                *
            "#,
        )
        .bind(&new_profile.name)
        .bind(&new_profile.handle)
        .bind(&new_profile.profile_img_url)
        .bind(&new_profile.bio)
        .bind(id)
        .fetch_one(&*self.conn)
        .await?;

        Ok(updated_user)
    }
}
