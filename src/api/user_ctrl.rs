use axum::{
    extract::{Path, Query, State},
    middleware,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    config::app_state::UserState,
    domain::{
        dto::{
            user::{RequestSignin, RequestSignup, RequestUpsertProfile},
            RequestCursorParmas, SuccessResponse,
        },
        model::jwt_claims::JwtClaims,
    },
    error::CustomError,
    middleware::auth_middleware::mw_require_auth,
    repository::{follow_repo::FollowRepository, user_repo::UserRepository},
    services::{follow_service::FollowService, user_service::UserService},
};

pub fn di(db_pool: &PgPool) -> UserState {
    let db_pool = Arc::new(db_pool.clone());

    let user_repo = Arc::new(UserRepository::new(db_pool.clone()));
    let follow_repo = Arc::new(FollowRepository::new(db_pool));

    let user_service = Arc::new(UserService::new(user_repo.clone(), follow_repo.clone()));
    let follow_service = Arc::new(FollowService::new(user_repo, follow_repo));

    UserState { user_service, follow_service }
}

pub async fn routes(state: UserState) -> Router {
    let accessible_router = Router::new()
        .route("/signup", post(signup))
        .route("/signin", post(signin))
        .route("/{handle}", get(get_user))
        .route("/{handle}/followers", get(list_user_follower))
        .route("/{handle}/following", get(list_user_following));

    let restricted_router = Router::new()
        .route("/me", get(me))
        .route("/me/profile", put(upsert_profile))
        .route("/{target_user_handle}/follow", delete(unfollow).post(follow))
        .layer(middleware::from_fn(mw_require_auth));

    let routes = accessible_router.merge(restricted_router).with_state(state);
    routes
}

// POST api/user/signup
pub async fn signup(
    State(state): State<UserState>,
    Json(signup_dto): Json<RequestSignup>,
) -> Result<impl IntoResponse, CustomError> {
    match state.user_service.signup(signup_dto).await {
        Ok(message) => Ok(Json(SuccessResponse::<String>::new(&message, None))),
        Err(err) => Err(err),
    }
}

// POST api/user/signin
pub async fn signin(
    State(state): State<UserState>,
    Json(signin_dto): Json<RequestSignin>,
) -> Result<impl IntoResponse, CustomError> {
    match state.user_service.signin(signin_dto).await {
        Ok(token) => Ok(Json(SuccessResponse::new("Success to login", Some(token)))),
        Err(err) => Err(err),
    }
}

// GET api/user/me
pub async fn me(
    State(state): State<UserState>,
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
    State(state): State<UserState>,
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
    State(state): State<UserState>,
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

// GET api/user/{handle}/followers
pub async fn list_user_follower(
    State(state): State<UserState>,
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
    State(state): State<UserState>,
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

// POST api/user/{target_user_handle}/follow
pub async fn follow(
    State(state): State<UserState>,
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

// DELETE api/user/{target_user_handle}/follow
pub async fn unfollow(
    State(state): State<UserState>,
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
