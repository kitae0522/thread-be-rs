use std::sync::Arc;

use crate::services::{thread_service::ThreadService, user_service::UserService};

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
}

#[derive(Clone)]
pub struct UserState {
    pub user_service: Arc<UserService>,
}

#[derive(Clone)]
pub struct ThreadState {
    pub thread_service: Arc<ThreadService>,
}
