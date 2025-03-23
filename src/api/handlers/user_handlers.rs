use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Extension, Json,
};

use crate::{
    api::state::AppState,
    domain::{
        dto::{user::RequestUpsertProfile, RequestCursorParmas, SuccessResponse},
        model::jwt_claims::JwtClaims,
    },
    error::CustomError,
    utils,
};

// GET api/user/me
pub async fn me(
    State(state): State<AppState>,
    Extension(token_context): Extension<JwtClaims>,
) -> Result<impl IntoResponse, CustomError> {
    match state.user_service.me(token_context.id).await {
        Ok(profile) => {
            Ok(Json(SuccessResponse::new("Success to fetch your profile", Some(profile))))
        }
        Err(err) => Err(err),
    }
}

// PUT api/user/me/profile
pub async fn upsert_profile(
    State(state): State<AppState>,
    Extension(token_context): Extension<JwtClaims>,
    Json(profile_dto): Json<RequestUpsertProfile>,
) -> Result<impl IntoResponse, CustomError> {
    match state.user_service.upsert_profile(token_context.id, profile_dto).await {
        Ok(profile) => Ok(Json(SuccessResponse::new(
            "Success to upsert your profile",
            Some(profile),
        ))),
        Err(err) => Err(err),
    }
}

// GET api/user/{handle}
pub async fn get_user_by_handle(
    State(state): State<AppState>,
    Path(handle): Path<String>,
) -> Result<impl IntoResponse, CustomError> {
    match state.user_service.get_user(&handle).await {
        Ok(profile) => Ok(Json(SuccessResponse::new(
            &format!("Success to fetch '{}' profile", handle),
            Some(profile),
        ))),
        Err(err) => Err(err),
    }
}

// GET api/user/{handle}/thread
pub async fn list_thread_by_user_handle(
    State(state): State<AppState>,
    Path(user_handle): Path<String>,
    Query(params): Query<RequestCursorParmas>,
) -> Result<impl IntoResponse, CustomError> {
    let (cursor, limit) =
        utils::cursor::preprocessing_cursor(params.cursor.as_deref(), params.limit);
    let user_thread_list = state
        .thread_service
        .list_thread_by_user_handle(&user_handle, cursor, limit)
        .await?;
    Ok(Json(SuccessResponse::new(
        "Success to fetch user based thread list",
        Some(user_thread_list),
    )))
}
