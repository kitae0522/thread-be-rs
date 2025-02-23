use dotenvy::dotenv;
use std::sync::OnceLock;

pub fn envs() -> &'static Envs {
    static INSTANCE: OnceLock<Envs> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        dotenv().ok();
        Envs::new()
    })
}

#[derive(Debug)]
pub struct Envs {
    pub db_url: String,
    pub jwt_secret: String,
    pub jwt_expiration_in_seconds: i64,
}

impl Envs {
    pub fn new() -> Self {
        Self {
            db_url: get_env("DATABASE_URL", "sqlite:./sqlite.db"),
            jwt_secret: get_env("JWT_SECRET", "tempSecret"),
            jwt_expiration_in_seconds: get_env_as_int(
                "JWT_EXPIRATION_IN_SECONDS",
                60 * 60 * 24 * 7,
            ),
        }
    }
}

fn get_env(key: &str, fallback: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| fallback.to_string())
}

fn get_env_as_int(key: &str, fallback: i64) -> i64 {
    std::env::var(key).ok().and_then(|val| val.parse().ok()).unwrap_or(fallback)
}
