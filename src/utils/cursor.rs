use crate::domain::model::cursor_claims::CursorClaims;

pub fn preprocessing_cursor(
    cursor: Option<&str>,
    limit: Option<i64>,
) -> (CursorClaims, i64) {
    let cursor = cursor.unwrap_or_default();
    let claims = CursorClaims::decode_cursor(cursor).unwrap_or_default();
    let limit = limit.unwrap_or(10);

    (claims, limit)
}
