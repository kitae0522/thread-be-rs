use crate::error::CustomError;

pub mod follow_repo;
pub mod thread_repo;
pub mod user_repo;
pub mod views_repo;
pub mod votes_repo;

pub type RepositoryResult<T> = Result<T, CustomError>;
