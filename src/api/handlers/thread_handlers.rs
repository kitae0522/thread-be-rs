use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Extension, Json,
};

use crate::{
    api::state::AppState,
    domain::{
        dto::{
            thread::{RequestCreateThread, RequestUpdateThread},
            RequestCursorParmas, SuccessResponse,
        },
        model::jwt_claims::JwtClaims,
    },
    error::CustomError,
    utils,
};

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

// GET api/thread/{id}
pub async fn get_thread_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, CustomError> {
    let thread = state.thread_service.get_thread_by_id(id).await?;
    Ok(Json(SuccessResponse::new("Success to fetch thread", Some(thread))))
}

// GET api/thread/{id}/subthread
pub async fn list_subthread_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Query(params): Query<RequestCursorParmas>,
) -> Result<impl IntoResponse, CustomError> {
    let (cursor, limit) =
        utils::cursor::preprocessing_cursor(params.cursor.as_deref(), params.limit);
    let thread =
        state.thread_service.list_subthread_by_parent_id(id, cursor, limit).await?;
    Ok(Json(SuccessResponse::new("Success to fetch thread", Some(thread))))
}

// GET api/thread/feed/guest
pub async fn list_guest_feed_thread(
    State(state): State<AppState>,
    Query(params): Query<RequestCursorParmas>,
) -> Result<impl IntoResponse, CustomError> {
    let (cursor, limit) =
        utils::cursor::preprocessing_cursor(params.cursor.as_deref(), params.limit);
    let guest_thread_list =
        state.thread_service.list_recommend_thread(None, cursor, limit).await?;
    Ok(Json(SuccessResponse::new(
        "Success to fetch thread list",
        Some(guest_thread_list),
    )))
}

// GET api/thread/feed/personal
pub async fn list_personal_feed_thread(
    State(state): State<AppState>,
    Extension(token_context): Extension<JwtClaims>,
    Query(params): Query<RequestCursorParmas>,
) -> Result<impl IntoResponse, CustomError> {
    let (cursor, limit) =
        utils::cursor::preprocessing_cursor(params.cursor.as_deref(), params.limit);
    let personal_thread_list = state
        .thread_service
        .list_recommend_thread(Some(token_context.id), cursor, limit)
        .await?;
    Ok(Json(SuccessResponse::new(
        "Success to fetch thread list",
        Some(personal_thread_list),
    )))
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
