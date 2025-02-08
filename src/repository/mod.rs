use crate::error::CustomError;

pub mod follow_repo;
pub mod thread_repo;
pub mod user_repo;

pub type RepositoryResult<T> = Result<T, CustomError>;
