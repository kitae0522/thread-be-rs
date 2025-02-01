use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

// NOTE: Token Max Age = One Week (per seconds)
const TOKEN_MAX_AGE: i64 = 60 * 60 * 24 * 7;

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
    pub fn new(user_id: i64, user_email: &str) -> String {
        let time_now = Utc::now();
        let issued_at = time_now.timestamp() as u64;
        let expiration =
            (time_now + Duration::seconds(TOKEN_MAX_AGE)).timestamp() as u64;

        let claims = JwtClaims {
            iat: issued_at,
            exp: expiration,
            id: user_id,
            email: user_email.to_string(),
        };

        // TODO: modify token secret variable
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("sample secret key".as_ref()),
        )
        .unwrap();

        token
    }
}
