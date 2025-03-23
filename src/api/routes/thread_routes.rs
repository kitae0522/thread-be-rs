use axum::{
    middleware,
    routing::{get, post, put},
    Router,
};

use crate::{
    api::handlers::{
        thread_handlers::{
            create_thread, delete_thread, get_thread_by_id, list_guest_feed_thread,
            list_personal_feed_thread, list_subthread_by_id, update_thread,
        },
        votes_handlers::{
            cancel_downvote_thread, cancel_upvote_thread, downvote_thread, upvote_thread,
        },
    },
    api::middleware::auth_middleware::mw_require_auth,
    api::state::AppState,
};

pub fn routes() -> Router<AppState> {
    let accessible_router = Router::new()
        .route("/feed/guest", get(list_guest_feed_thread))
        .route("/{id}", get(get_thread_by_id))
        .route("/{id}/subthread", get(list_subthread_by_id));

    let restricted_router = Router::new()
        .route("/", post(create_thread))
        .route("/feed/personal", get(list_personal_feed_thread))
        .route("/{id}", put(update_thread).delete(delete_thread))
        .route("/{id}/up", post(upvote_thread).delete(cancel_upvote_thread))
        .route("/{id}/down", post(downvote_thread).delete(cancel_downvote_thread))
        .layer(middleware::from_fn(mw_require_auth));

    let routes = accessible_router.merge(restricted_router);
    routes
}
