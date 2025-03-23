use axum::{
    middleware,
    routing::{delete, get, put},
    Router,
};

use crate::{
    api::handlers::{
        follow_handlers::{follow, list_user_follower, list_user_following, unfollow},
        user_handlers::{
            get_user_by_handle, list_thread_by_user_handle, me, upsert_profile,
        },
        votes_handlers::{list_downvoted_thread, list_upvoted_thread},
    },
    api::middleware::auth_middleware::mw_require_auth,
    api::state::AppState,
};

pub fn routes() -> Router<AppState> {
    let accessible_router = Router::new()
        .route("/{handle}", get(get_user_by_handle))
        .route("/{handle}/thread", get(list_thread_by_user_handle))
        .route("/{handle}/followers", get(list_user_follower))
        .route("/{handle}/following", get(list_user_following));

    let restricted_router = Router::new()
        .route("/me", get(me))
        .route("/me/profile", put(upsert_profile))
        .route("/me/thread/upvoted", get(list_upvoted_thread))
        .route("/me/thread/downvoted", get(list_downvoted_thread))
        .route("/{target_user_handle}/follow", delete(unfollow).post(follow))
        .layer(middleware::from_fn(mw_require_auth));

    let routes = accessible_router.merge(restricted_router);
    routes
}
