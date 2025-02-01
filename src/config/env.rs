use dotenvy::dotenv;

#[derive(Debug)]
pub struct Envs {
    pub db_url: String,
    pub jwt_secret: String,
    pub jwt_expiration_in_seconds: i64,
    // pub redis_host: String,
    // pub redis_password: String,
    // pub redis_db: i64,
}

impl Envs {
    pub fn new() -> Self {
        Envs {
            db_url: get_env("DATABASE_URL", "sqlite:./sqlite.db"),
            jwt_secret: get_env("JWT_SECRET", "tempSecret"),
            jwt_expiration_in_seconds: get_env_as_int(
                "JWT_EXPIRATION_IN_SECONDS",
                60 * 60 * 24 * 7,
            ),
            // redis_host: get_env("REDIS_HOST", "tempHost"),
            // redis_password: get_env("REDIS_PASSWORD", "tempPassword"),
            // redis_db: get_env_as_int("REDIS_DB", 0),
        }
    }
}

fn get_env(key: &str, fallback: &str) -> String {
    match std::env::var(key) {
        Ok(value) => value,
        Err(_) => fallback.to_string(),
    }
}

fn get_env_as_int(key: &str, fallback: i64) -> i64 {
    match std::env::var(key) {
        Ok(value) => value.parse::<i64>().unwrap_or(fallback),
        Err(_) => fallback,
    }
}
