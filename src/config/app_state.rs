use std::sync::Arc;

use crate::services::{
    follow_service::FollowService, thread_service::ThreadService,
    user_service::UserService, votes_service::VotesService,
};

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
    pub thread_service: Arc<ThreadService>,
    pub follow_service: Arc<FollowService>,
    pub votes_service: Arc<VotesService>,
}
