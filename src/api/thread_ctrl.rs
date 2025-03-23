use axum::{
    extract::{Path, Query, State},
    middleware,
    response::IntoResponse,
    routing::{get, post, put},
    Extension, Json, Router,
};

use crate::{
    config::app_state::AppState,
    domain::{
        dto::{
            thread::{RequestCreateThread, RequestUpdateThread},
            RequestCursorParmas, SuccessResponse,
        },
        model::{jwt_claims::JwtClaims, votes::ReactionType},
    },
    error::CustomError,
    middleware::auth_middleware::mw_require_auth,
};

pub async fn routes(state: AppState) -> Router {
    let accessible_router = Router::new()
        .route("/feed/guest", get(guest_feed_thread))
        .route("/{id}", get(get_thread_by_id));

    let restricted_router = Router::new()
        .route("/", post(create_thread))
        .route("/{id}", put(update_thread).delete(delete_thread))
        .route("/{id}/up", post(like_thread).delete(cancel_like_thread))
        .route("/{id}/down", post(dislike_thread).delete(cancel_dislike_thread))
        .route("/feed/personal", get(personal_feed_thread))
        .layer(middleware::from_fn(mw_require_auth));

    let routes = accessible_router.merge(restricted_router).with_state(state);
    routes
}

// POST api/thread
pub async fn create_thread(
    State(state): State<AppState>,
    Extension(token_context): Extension<JwtClaims>,
    Json(create_thread_dto): Json<RequestCreateThread>,
) -> Result<impl IntoResponse, CustomError> {
    let new_thread =
        state.thread_service.create_thread(token_context.id, create_thread_dto).await?;
    Ok(Json(SuccessResponse::new("Success to create thread", Some(new_thread))))
}

// GET api/thread/feed/guest
pub async fn guest_feed_thread(
    State(state): State<AppState>,
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
    State(state): State<AppState>,
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
    State(state): State<AppState>,
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
    State(state): State<AppState>,
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
    State(state): State<AppState>,
    Extension(token_context): Extension<JwtClaims>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, CustomError> {
    let _ok = state.thread_service.delete_thread_by_id(token_context.id, id).await?;
    Ok(Json(SuccessResponse::<String>::new("Success to delete thread", None)))
}

// POST api/thread/{id}/up
pub async fn like_thread(
    State(state): State<AppState>,
    Extension(token_context): Extension<JwtClaims>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, CustomError> {
    let _ok = state.votes_service.react(token_context.id, id, ReactionType::Up).await?;
    Ok(Json(SuccessResponse::<String>::new("Success to like thread", None)))
}

// DELETE api/thread/{id}/up
pub async fn cancel_like_thread(
    State(state): State<AppState>,
    Extension(token_context): Extension<JwtClaims>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, CustomError> {
    let _ok =
        state.votes_service.react_cancel(token_context.id, id, ReactionType::Up).await?;
    Ok(Json(SuccessResponse::<String>::new("Success to cancel thread", None)))
}

// POST api/thread/{id}/down
pub async fn dislike_thread(
    State(state): State<AppState>,
    Extension(token_context): Extension<JwtClaims>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, CustomError> {
    let _ok = state.votes_service.react(token_context.id, id, ReactionType::Down).await?;
    Ok(Json(SuccessResponse::<String>::new("Success to dislike thread", None)))
}

// DELETE api/thread/{id}/down
pub async fn cancel_dislike_thread(
    State(state): State<AppState>,
    Extension(token_context): Extension<JwtClaims>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, CustomError> {
    let _ok = state
        .votes_service
        .react_cancel(token_context.id, id, ReactionType::Down)
        .await?;
    Ok(Json(SuccessResponse::<String>::new("Success to cancel thread", None)))
}
