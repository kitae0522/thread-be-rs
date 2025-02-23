use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Error, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::config;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JwtClaims {
    // issued at, expiration
    iat: u64,
    exp: u64,

    // data
    pub id: i64,
    pub email: String,
}

impl JwtClaims {
    pub fn new(user_id: i64, user_email: &str) -> Self {
        let time_now = Utc::now();
        let issued_at = time_now.timestamp() as u64;
        let expiration = (time_now
            + Duration::seconds(config::env::envs().jwt_expiration_in_seconds))
        .timestamp() as u64;

        JwtClaims {
            iat: issued_at,
            exp: expiration,
            id: user_id,
            email: user_email.to_string(),
        }
    }

    pub fn encode_jwt(claims: JwtClaims) -> Result<String, Error> {
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(config::env::envs().jwt_secret.as_ref()),
        )
        .map_err(|err| {
            error!("Error encoding JWT: {}", err);
            err
        })
    }

    pub fn decode_jwt(token: &str) -> Result<JwtClaims, Error> {
        let decoding_key =
            DecodingKey::from_secret(config::env::envs().jwt_secret.as_ref());
        decode::<JwtClaims>(token, &decoding_key, &Validation::default())
            .map(|decoded_token| decoded_token.claims)
            .map_err(|err| {
                error!("Error decoding JWT: {}", err);
                err
            })
    }
}
