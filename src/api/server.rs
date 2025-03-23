use std::sync::Arc;

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
    api::middleware::log_middleware::mw_logging_request,
    api::routes::{auth_routes, thread_routes, user_routes},
    api::state::AppState,
    domain::dto::ErrorResponse,
    repository::{
        follow_repo::FollowRepository, thread_repo::ThreadRepository,
        user_repo::UserRepository, views_repo::ViewsRepository,
        votes_repo::VotesRepository,
    },
    services::{
        follow_service::FollowService, thread_service::ThreadService,
        user_service::UserService, votes_service::VotesService,
    },
};

fn di(db_pool: &PgPool) -> AppState {
    let db_pool = Arc::new(db_pool.clone());

    let user_repo = Arc::new(UserRepository::new(Arc::clone(&db_pool)));
    let thread_repo = Arc::new(ThreadRepository::new(Arc::clone(&db_pool)));
    let follow_repo = Arc::new(FollowRepository::new(Arc::clone(&db_pool)));
    let votes_repo = Arc::new(VotesRepository::new(Arc::clone(&db_pool)));
    let views_repo = Arc::new(ViewsRepository::new(Arc::clone(&db_pool)));

    let user_service = Arc::new(UserService::new(user_repo.clone(), follow_repo.clone()));
    let thread_service = Arc::new(ThreadService::new(
        user_repo.clone(),
        thread_repo.clone(),
        votes_repo.clone(),
        views_repo.clone(),
    ));
    let follow_service =
        Arc::new(FollowService::new(user_repo.clone(), follow_repo.clone()));
    let votes_service =
        Arc::new(VotesService::new(user_repo.clone(), thread_repo.clone(), votes_repo));

    AppState { user_service, thread_service, follow_service, votes_service }
}

pub async fn routes_all(db_pool: &PgPool) -> Router {
    let app_state = di(db_pool);

    let router_all = Router::new()
        .route("/ping", get(health_check_handler))
        .nest("/auth", auth_routes::routes())
        .nest("/user", user_routes::routes())
        .nest("/thread", thread_routes::routes())
        .with_state(app_state);

    let app = Router::new()
        .nest("/api", router_all)
        .layer(middleware::from_fn(mw_logging_request))
        .fallback(fallback_handler);
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
