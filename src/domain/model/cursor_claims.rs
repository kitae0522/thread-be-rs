use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CursorClaims {
    pub id: i64,
    pub created_at: Option<DateTime<Utc>>,
}

impl Default for CursorClaims {
    fn default() -> Self {
        let created_default_str = Utc::now().to_rfc3339();
        let created_at = DateTime::parse_from_rfc3339(&created_default_str)
            .ok()
            .map(|dt| dt.with_timezone(&Utc));
        Self { id: 0, created_at }
    }
}

impl CursorClaims {
    pub fn decode_cursor(cursor: &str) -> Option<CursorClaims> {
        let decoded_bytes = general_purpose::STANDARD.decode(cursor).ok()?;
        let decoded_str = str::from_utf8(&decoded_bytes).ok()?;
        let claims: serde_json::Value = serde_json::from_str(decoded_str).ok()?;

        let id = claims.get("id").and_then(|v| v.as_i64()).unwrap_or_default();
        let created_at = claims
            .get("created_at")
            .and_then(|v| v.as_str())
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        Some(CursorClaims { id, created_at })
    }
}
