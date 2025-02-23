use std::sync::Arc;

use axum::{
    extract::{Path, State},
    middleware,
    response::IntoResponse,
    routing::{delete, post},
    Extension, Json, Router,
};
use sqlx::PgPool;

use crate::{
    config::app_state::FollowState,
    domain::{dto::SuccessResponse, model::jwt_claims::JwtClaims},
    error::CustomError,
    middleware::auth_middleware::mw_require_auth,
    repository::{follow_repo::FollowRepository, user_repo::UserRepository},
    services::follow_service::FollowService,
};

pub fn di(db_pool: &PgPool) -> FollowState {
    let db_pool = Arc::new(db_pool.clone());

    let user_repo = Arc::new(UserRepository { conn: db_pool.clone() });
    let follow_repo = Arc::new(FollowRepository { conn: db_pool });
    let follow_service = Arc::new(FollowService::new(user_repo, follow_repo));

    FollowState { follow_service }
}

pub fn routes(state: FollowState) -> Router {
    let accessible_router = Router::new().with_state(state.clone());

    let restricted_router = Router::new()
        .route("/{target_user_handle}", post(follow).delete(unfollow))
        .layer(middleware::from_fn(mw_require_auth))
        .with_state(state);

    let routes = accessible_router.merge(restricted_router);
    routes
}

// POST api/follow/{target_user_handle}
pub async fn follow(
    State(state): State<FollowState>,
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

// DELETE api/follow/{target_user_handle}
pub async fn unfollow(
    State(state): State<FollowState>,
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
