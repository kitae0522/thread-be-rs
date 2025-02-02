use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CursorClaims {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub created_at: Option<DateTime<Utc>>,
}

impl CursorClaims {
    pub fn decode_cursor(cursor: &str) -> Option<CursorClaims> {
        let decoded_bytes = general_purpose::STANDARD.decode(cursor).ok()?;
        let decoded_str = str::from_utf8(&decoded_bytes).ok()?;
        let claims: serde_json::Value = serde_json::from_str(decoded_str).ok()?;

        let id = claims.get("id").and_then(|v| v.as_i64());
        let user_id = claims.get("user_id").and_then(|v| v.as_i64());
        let created_at = claims
            .get("created_at")
            .and_then(|v| v.as_str())
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        Some(CursorClaims { id, user_id, created_at })
    }
}
