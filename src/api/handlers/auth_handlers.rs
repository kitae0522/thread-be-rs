use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    api::state::AppState,
    domain::dto::{
        user::{RequestSignin, RequestSignup},
        SuccessResponse,
    },
    error::CustomError,
};

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
