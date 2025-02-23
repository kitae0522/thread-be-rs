use axum::{
    extract::{Path, Query, State},
    middleware,
    response::IntoResponse,
    routing::{get, post, put},
    Extension, Json, Router,
};
use sqlx::PgPool;
use std::sync::Arc;

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
    repository::{follow_repo::FollowRepository, user_repo::UserRepository},
    services::user_service::UserService,
};

pub fn di(db_pool: &PgPool) -> AppState {
    let db_pool = Arc::new(db_pool.clone());

    let user_repo = Arc::new(UserRepository { conn: db_pool.clone() });
    let follow_repo = Arc::new(FollowRepository { conn: db_pool });
    let user_service = Arc::new(UserService::new(user_repo, follow_repo));

    AppState { user_service }
}

pub async fn routes(state: AppState) -> Router {
    let accessible_router = Router::new()
        .route("/signup", post(signup))
        .route("/signin", post(signin))
        .with_state(state.clone());

    let restricted_router = Router::new()
        .route("/profile", get(me))
        .route("/profile", put(upsert_profile))
        .route("/{handle}", get(get_user))
        .route("/{handle}/followers", get(list_user_follower))
        .route("/{handle}/following", get(list_user_following))
        .layer(middleware::from_fn(mw_require_auth))
        .with_state(state);

    let routes = accessible_router.merge(restricted_router);
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

// GET api/user/profile
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

// PUT api/user/profile
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
