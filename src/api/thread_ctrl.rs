use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    middleware,
    response::IntoResponse,
    routing::{get, post, put},
    Extension, Json, Router,
};
use sqlx::PgPool;

use crate::{
    config::app_state::ThreadState,
    domain::{
        dto::{
            thread::{RequestCreateThread, RequestUpdateThread},
            RequestCursorParmas, SuccessResponse,
        },
        model::jwt_claims::JwtClaims,
    },
    error::CustomError,
    middleware::auth_middleware::mw_require_auth,
    repository::{thread_repo::ThreadRepository, user_repo::UserRepository},
    services::thread_service::ThreadService,
};

pub fn di(db_pool: &PgPool) -> ThreadState {
    let db_pool = Arc::new(db_pool.clone());

    let user_repo = Arc::new(UserRepository::new(db_pool.clone()));
    let thread_repo = Arc::new(ThreadRepository::new(db_pool));

    let thread_service = Arc::new(ThreadService::new(user_repo, thread_repo));

    ThreadState { thread_service }
}

pub async fn routes(state: ThreadState) -> Router {
    let accessible_router = Router::new()
        .route("/feed/guest", get(guest_feed_thread))
        .route("/{id}", get(get_thread_by_id));

    let restricted_router = Router::new()
        .route("/", post(create_thread))
        .route("/{id}", put(update_thread).delete(delete_thread))
        .route("/feed/personal", get(personal_feed_thread))
        .route("/user/{user_handle}", get(list_thread_by_user_handle))
        .layer(middleware::from_fn(mw_require_auth));

    let routes = accessible_router.merge(restricted_router).with_state(state);
    routes
}

// POST api/thread
pub async fn create_thread(
    State(state): State<ThreadState>,
    Extension(token_context): Extension<JwtClaims>,
    Json(create_thread_dto): Json<RequestCreateThread>,
) -> Result<impl IntoResponse, CustomError> {
    let new_thread =
        state.thread_service.create_thread(token_context.id, create_thread_dto).await?;
    Ok(Json(SuccessResponse::new("Success to create thread", Some(new_thread))))
}

// GET api/thread/feed/guest
pub async fn guest_feed_thread(
    State(state): State<ThreadState>,
    Query(params): Query<RequestCursorParmas>,
) -> Result<impl IntoResponse, CustomError> {
    let guest_thread_list = state
        .thread_service
        .list_recommend_thread(None, params.cursor.as_deref(), params.limit)
        .await?;
    Ok(Json(SuccessResponse::new(
        "Success to fetch thread list",
        Some(guest_thread_list),
    )))
}

// GET api/thread/feed/personal
pub async fn personal_feed_thread(
    State(state): State<ThreadState>,
    Extension(token_context): Extension<JwtClaims>,
    Query(params): Query<RequestCursorParmas>,
) -> Result<impl IntoResponse, CustomError> {
    let personal_thread_list = state
        .thread_service
        .list_recommend_thread(
            Some(token_context.id),
            params.cursor.as_deref(),
            params.limit,
        )
        .await?;
    Ok(Json(SuccessResponse::new(
        "Success to fetch thread list",
        Some(personal_thread_list),
    )))
}

// GET api/thread/{id}
pub async fn get_thread_by_id(
    State(state): State<ThreadState>,
    Path(id): Path<i64>,
    Query(params): Query<RequestCursorParmas>,
) -> Result<impl IntoResponse, CustomError> {
    let thread = state
        .thread_service
        .get_thread_by_id(id, params.cursor.as_deref(), params.limit)
        .await?;
    Ok(Json(SuccessResponse::new("Success to fetch thread", Some(thread))))
}

// PUT api/thread/{id}
pub async fn update_thread(
    State(state): State<ThreadState>,
    Extension(token_context): Extension<JwtClaims>,
    Path(id): Path<i64>,
    Json(update_thread_dto): Json<RequestUpdateThread>,
) -> Result<impl IntoResponse, CustomError> {
    let thread = state
        .thread_service
        .update_thread_by_id(token_context.id, id, update_thread_dto)
        .await?;
    Ok(Json(SuccessResponse::new("Success to update thread", Some(thread))))
}

// DELETE api/thread/{id}
pub async fn delete_thread(
    State(state): State<ThreadState>,
    Extension(token_context): Extension<JwtClaims>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, CustomError> {
    let _ok = state.thread_service.delete_thread_by_id(token_context.id, id).await?;
    Ok(Json(SuccessResponse::<String>::new("Success to delete thread", None)))
}

// GET api/thread/user/{user_handle}
pub async fn list_thread_by_user_handle(
    State(state): State<ThreadState>,
    Path(user_handle): Path<String>,
    Query(params): Query<RequestCursorParmas>,
) -> Result<impl IntoResponse, CustomError> {
    let user_thread_list = state
        .thread_service
        .list_thread_by_user_handle(&user_handle, params.cursor.as_deref(), params.limit)
        .await?;
    Ok(Json(SuccessResponse::new(
        "Success to fetch user based thread list",
        Some(user_thread_list),
    )))
}
