use crate::{domain::model::jwt_claims::JwtClaims, error::CustomError};
use axum::{body::Body, http::Request, middleware::Next, response::IntoResponse};
use jsonwebtoken::{decode, errors::Error, DecodingKey, Validation};
use tracing::error;

const SECRET_KEY: &str = "sample secret key";

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
    match decode_jwt(token).await {
        Ok(payload) => {
            req.extensions_mut().insert(payload);
            return next.run(req).await;
        }
        Err(err) => {
            println!("JWT decode error: {}", err);
            return CustomError::Unauthorized("Invalid or expired token".to_string())
                .into_response();
        }
    }
}

async fn decode_jwt(token: &str) -> Result<JwtClaims, Error> {
    let decoding_key = DecodingKey::from_secret(SECRET_KEY.as_ref());
    decode::<JwtClaims>(token, &decoding_key, &Validation::default())
        .map(|decoded_token| decoded_token.claims)
        .map_err(|err| {
            error!("Error decoding JWT: {}", err);
            err
        })
}
