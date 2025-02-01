use bcrypt::{hash, verify};

use crate::error::CustomError;

// NOTE: Hashing cost (recommended range: 12 <= cost <= 14) for strong hashing security.
// TODO: Move this to an environment variable.
const HASHING_COST: u32 = 13;

pub fn hash_password(password: &str) -> Result<String, CustomError> {
    let hashed_password = hash(password, HASHING_COST)
        .map_err(|_| CustomError::InternalError("Password hashing failed".to_string()))?;
    Ok(hashed_password)
}

pub fn verify_password(
    password: &str,
    hashed_password: &str,
) -> Result<bool, CustomError> {
    let password_is_valid = verify(password, &hashed_password).map_err(|_| {
        CustomError::InternalError("Password verification failed".to_string())
    })?;

    if password_is_valid {
        return Ok(true);
    }
    Err(CustomError::InvalidCredentials)
}
