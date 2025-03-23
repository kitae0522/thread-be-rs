use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Extension, Json,
};

use crate::{
    api::state::AppState,
    domain::{
        dto::{RequestCursorParmas, SuccessResponse},
        model::{jwt_claims::JwtClaims, votes::ReactionType},
    },
    error::CustomError,
    utils,
};

// POST api/thread/{id}/up
pub async fn upvote_thread(
    state: State<AppState>,
    token_context: Extension<JwtClaims>,
    id: Path<i64>,
) -> Result<impl IntoResponse, CustomError> {
    vote_thread(state, token_context, id, ReactionType::Up).await
}

// DELETE api/thread/{id}/up
pub async fn cancel_upvote_thread(
    state: State<AppState>,
    token_context: Extension<JwtClaims>,
    id: Path<i64>,
) -> Result<impl IntoResponse, CustomError> {
    cancel_vote_thread(state, token_context, id, ReactionType::Up).await
}

// POST api/thread/{id}/down
pub async fn downvote_thread(
    state: State<AppState>,
    token_context: Extension<JwtClaims>,
    id: Path<i64>,
) -> Result<impl IntoResponse, CustomError> {
    vote_thread(state, token_context, id, ReactionType::Down).await
}

// DELETE api/thread/{id}/down
pub async fn cancel_downvote_thread(
    state: State<AppState>,
    token_context: Extension<JwtClaims>,
    id: Path<i64>,
) -> Result<impl IntoResponse, CustomError> {
    cancel_vote_thread(state, token_context, id, ReactionType::Down).await
}

// GET api/user/me/upvoted
pub async fn list_upvoted_thread(
    State(state): State<AppState>,
    Extension(token_context): Extension<JwtClaims>,
    Query(params): Query<RequestCursorParmas>,
) -> Result<impl IntoResponse, CustomError> {
    let (cursor, limit) =
        utils::cursor::preprocessing_cursor(params.cursor.as_deref(), params.limit);
    match state.thread_service.list_upvoted_thread(token_context.id, cursor, limit).await
    {
        Ok(upvoted_threads) => Ok(Json(SuccessResponse::new(
            "Success to fetch upvoted thread list",
            Some(upvoted_threads),
        ))),
        Err(err) => Err(err),
    }
}

// GET api/user/{handle}/downvoted
pub async fn list_downvoted_thread(
    State(state): State<AppState>,
    Extension(token_context): Extension<JwtClaims>,
    Query(params): Query<RequestCursorParmas>,
) -> Result<impl IntoResponse, CustomError> {
    let (cursor, limit) =
        utils::cursor::preprocessing_cursor(params.cursor.as_deref(), params.limit);
    match state
        .thread_service
        .list_downvoted_thread(token_context.id, cursor, limit)
        .await
    {
        Ok(downvoted_threads) => Ok(Json(SuccessResponse::new(
            "Success to fetch downvoted thread list",
            Some(downvoted_threads),
        ))),
        Err(err) => Err(err),
    }
}

async fn vote_thread(
    State(state): State<AppState>,
    Extension(token_context): Extension<JwtClaims>,
    Path(id): Path<i64>,
    reaction: ReactionType,
) -> Result<impl IntoResponse, CustomError> {
    let message = match reaction {
        ReactionType::Up => "Successfully upvoted the thread",
        ReactionType::Down => "Successfully downvoted the thread",
    };

    let _ok = state.votes_service.react(token_context.id, id, reaction).await?;
    Ok(Json(SuccessResponse::<String>::new(message, None)))
}

async fn cancel_vote_thread(
    State(state): State<AppState>,
    Extension(token_context): Extension<JwtClaims>,
    Path(id): Path<i64>,
    reaction: ReactionType,
) -> Result<impl IntoResponse, CustomError> {
    let message = match reaction {
        ReactionType::Up => "Successfully canceled upvote",
        ReactionType::Down => "Successfully canceled downvote",
    };

    let _ok = state.votes_service.react_cancel(token_context.id, id, reaction).await?;
    Ok(Json(SuccessResponse::<String>::new(message, None)))
}
