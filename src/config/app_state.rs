use std::sync::Arc;

use crate::services::{
    follow_service::FollowService, thread_service::ThreadService,
    user_service::UserService,
};

#[derive(Clone)]
pub struct UserState {
    pub user_service: Arc<UserService>,
    pub follow_service: Arc<FollowService>,
}

#[derive(Clone)]
pub struct ThreadState {
    pub thread_service: Arc<ThreadService>,
}
