use axum::{
    extract::{Path, Query, State},
    middleware,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};

use crate::{
    config::app_state::AppState,
    domain::{
        dto::{
            user::{RequestSignin, RequestSignup, RequestUpsertProfile},
            RequestCursorParmas, SuccessResponse,
        },
        model::jwt_claims::JwtClaims,
    },
    error::CustomError,
    middleware::auth_middleware::mw_require_auth,
};

pub async fn routes(state: AppState) -> Router {
    let accessible_router = Router::new()
        .route("/signup", post(signup))
        .route("/signin", post(signin))
        .route("/{handle}", get(get_user))
        .route("/{handle}/thread", get(list_thread_by_user_handle))
        .route("/{handle}/followers", get(list_user_follower))
        .route("/{handle}/following", get(list_user_following));

    let restricted_router = Router::new()
        .route("/me", get(me))
        .route("/me/profile", put(upsert_profile))
        .route("/me/thread/upvoted", get(list_upvoted_thread))
        .route("/me/thread/downvoted", get(list_downvoted_thread))
        .route("/{target_user_handle}/follow", delete(unfollow).post(follow))
        .layer(middleware::from_fn(mw_require_auth));

    let routes = accessible_router.merge(restricted_router).with_state(state);
    routes
}

// POST api/user/signup
pub async fn signup(
    State(state): State<AppState>,
    Json(signup_dto): Json<RequestSignup>,
) -> Result<impl IntoResponse, CustomError> {
    match state.user_service.signup(signup_dto).await {
        Ok(message) => Ok(Json(SuccessResponse::<String>::new(&message, None))),
        Err(err) => Err(err),
    }
}

// POST api/user/signin
pub async fn signin(
    State(state): State<AppState>,
    Json(signin_dto): Json<RequestSignin>,
) -> Result<impl IntoResponse, CustomError> {
    match state.user_service.signin(signin_dto).await {
        Ok(token) => Ok(Json(SuccessResponse::new("Success to login", Some(token)))),
        Err(err) => Err(err),
    }
}

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
pub async fn get_user(
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
    let user_thread_list = state
        .thread_service
        .list_thread_by_user_handle(&user_handle, params.cursor.as_deref(), params.limit)
        .await?;
    Ok(Json(SuccessResponse::new(
        "Success to fetch user based thread list",
        Some(user_thread_list),
    )))
}

// GET api/user/{handle}/followers
pub async fn list_user_follower(
    State(state): State<AppState>,
    Path(handle): Path<String>,
    Query(params): Query<RequestCursorParmas>,
) -> Result<impl IntoResponse, CustomError> {
    match state
        .user_service
        .list_user_follower(&handle, params.cursor.as_deref(), params.limit)
        .await
    {
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
    match state
        .user_service
        .list_user_following(&handle, params.cursor.as_deref(), params.limit)
        .await
    {
        Ok(following_list) => Ok(Json(SuccessResponse::new(
            "Success to fetch following list",
            Some(following_list),
        ))),
        Err(err) => Err(err),
    }
}

// GET api/user/me/upvoted
pub async fn list_upvoted_thread(
    State(state): State<AppState>,
    Extension(token_context): Extension<JwtClaims>,
    Query(params): Query<RequestCursorParmas>,
) -> Result<impl IntoResponse, CustomError> {
    match state
        .thread_service
        .list_upvoted_thread(
            token_context.id,
            params.cursor.as_deref(),
            params.limit,
        )
        .await
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
    match state
        .thread_service
        .list_downvoted_thread(
            token_context.id,
            params.cursor.as_deref(),
            params.limit,
        )
        .await
    {
        Ok(downvoted_threads) => Ok(Json(SuccessResponse::new(
            "Success to fetch downvoted thread list",
            Some(downvoted_threads),
        ))),
        Err(err) => Err(err),
    }
}

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
