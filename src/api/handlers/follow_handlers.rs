use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Extension, Json,
};

use crate::{
    api::state::AppState,
    domain::{
        dto::{RequestCursorParmas, SuccessResponse},
        model::jwt_claims::JwtClaims,
    },
    error::CustomError,
    utils,
};

// POST api/user/{target_user_handle}/follow
pub async fn follow(
    State(state): State<AppState>,
    Extension(token_context): Extension<JwtClaims>,
    Path(target_user_handle): Path<String>,
) -> Result<impl IntoResponse, CustomError> {
    match state.follow_service.follow(token_context.id, &target_user_handle).await {
        Ok(success) => {
            if success {
                return Ok(Json(SuccessResponse::<String>::new(
                    &"Success to follow user",
                    None,
                )));
            }
            Err(CustomError::InternalError("Failed to follow user".to_string()))
        }
        Err(err) => Err(err),
    }
}

// DELETE api/user/{target_user_handle}/unfollow
pub async fn unfollow(
    State(state): State<AppState>,
    Extension(token_context): Extension<JwtClaims>,
    Path(target_user_handle): Path<String>,
) -> Result<impl IntoResponse, CustomError> {
    match state.follow_service.unfollow(token_context.id, &target_user_handle).await {
        Ok(success) => {
            if success {
                return Ok(Json(SuccessResponse::<String>::new(
                    &"Success to unfollow user",
                    None,
                )));
            }
            Err(CustomError::InternalError("Failed to unfollow user".to_string()))
        }
        Err(err) => Err(err),
    }
}

// GET api/user/{handle}/followers
pub async fn list_user_follower(
    State(state): State<AppState>,
    Path(handle): Path<String>,
    Query(params): Query<RequestCursorParmas>,
) -> Result<impl IntoResponse, CustomError> {
    let (cursor, limit) =
        utils::cursor::preprocessing_cursor(params.cursor.as_deref(), params.limit);
    match state.follow_service.list_user_follower(&handle, cursor, limit).await {
        Ok(thread_list) => Ok(Json(SuccessResponse::new(
            "Success to fetch follower list",
            Some(thread_list),
        ))),
        Err(err) => Err(err),
    }
}

// GET api/user/{handle}/following
pub async fn list_user_following(
    State(state): State<AppState>,
    Path(handle): Path<String>,
    Query(params): Query<RequestCursorParmas>,
) -> Result<impl IntoResponse, CustomError> {
    let (cursor, limit) =
        utils::cursor::preprocessing_cursor(params.cursor.as_deref(), params.limit);
    match state.follow_service.list_user_following(&handle, cursor, limit).await {
        Ok(following_list) => Ok(Json(SuccessResponse::new(
            "Success to fetch following list",
            Some(following_list),
        ))),
        Err(err) => Err(err),
    }
}
