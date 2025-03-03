use axum::{
    http::StatusCode,
    middleware,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde_json::json;
use sqlx::PgPool;
use tracing::error;

use crate::{
    api::{thread_ctrl, user_ctrl},
    domain::dto::ErrorResponse,
    middleware::log_middleware::mw_logging_request,
};

pub async fn routes_all(db_pool: &PgPool) -> Router {
    let user_state = user_ctrl::di(db_pool);
    let thread_state = thread_ctrl::di(db_pool);

    let router_all = Router::new()
        .route("/ping", get(health_check_handler))
        .nest("/user", user_ctrl::routes(user_state).await)
        .nest("/thread", thread_ctrl::routes(thread_state).await)
        .fallback(fallback_handler);

    let app = Router::new()
        .nest("/api", router_all)
        .layer(middleware::from_fn(mw_logging_request));
    app
}

async fn health_check_handler() -> impl IntoResponse {
    Json(json!({"message": "pong"}))
}

async fn fallback_handler() -> impl IntoResponse {
    let status_code = StatusCode::NOT_FOUND;
    let message = "Handler Not Found";

    error!("status code: {}, message: {}", status_code, message);
    (status_code, Json(ErrorResponse::new(message, None))).into_response()
}
