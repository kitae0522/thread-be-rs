use axum::{
    http::StatusCode,
    middleware,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    api::{follow_ctrl, thread_ctrl, user_ctrl},
    middleware::log_middleware::mw_logging_request,
};

pub async fn routes_all(db_pool: &PgPool) -> Router {
    let user_state = user_ctrl::di(db_pool);
    let thread_state = thread_ctrl::di(db_pool);
    let follow_state = follow_ctrl::di(db_pool);

    let router_all = Router::new()
        .route("/ping", get(health_check_handler))
        .nest("/user", user_ctrl::routes(user_state.clone()).await)
        .nest("/thread", thread_ctrl::routes(thread_state.clone()).await)
        .nest("/follow", follow_ctrl::routes(follow_state.clone()))
        .fallback(fallback_handler);

    let app = Router::new()
        .nest("/api", router_all)
        .layer(middleware::from_fn(mw_logging_request));
    app
}

async fn health_check_handler() -> impl IntoResponse {
    println!("->> {:<12} - health_check_handler", "HANDLER");
    Json(json!({"message": "pong"}))
}

async fn fallback_handler() -> impl IntoResponse {
    println!("->> {:<12} - fallback_handler", "HANDLER");
    StatusCode::NOT_FOUND
}
