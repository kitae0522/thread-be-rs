use crate::{domain::model::jwt_claims::JwtClaims, error::CustomError};
use axum::{body::Body, http::Request, middleware::Next, response::IntoResponse};
use tracing::error;

pub async fn mw_require_auth(mut req: Request<Body>, next: Next) -> impl IntoResponse {
    let auth_header = req.headers().get("Authorization");
    if auth_header.is_none() {
        return CustomError::Forbidden("Authorization header is missing".to_string())
            .into_response();
    }

    let auth_str = match auth_header.unwrap().to_str() {
        Ok(auth_str) => auth_str,
        Err(_) => {
            return CustomError::Forbidden(
                "Invalid Authorization header format".to_string(),
            )
            .into_response();
        }
    };

    if !auth_str.starts_with("Bearer ") {
        return CustomError::Forbidden(
            "Authorization header must be in Bearer format".to_string(),
        )
        .into_response();
    }

    let token = &auth_str[7..];
    match JwtClaims::decode_jwt(token) {
        Ok(payload) => {
            req.extensions_mut().insert(payload);
            return next.run(req).await;
        }
        Err(err) => {
            error!("JWT decode error: {}", err);
            return CustomError::Unauthorized("Invalid or expired token".to_string())
                .into_response();
        }
    }
}
