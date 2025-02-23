use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

mod api;
mod config;
mod domain;
mod error;
mod middleware;
mod repository;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config::env::envs().db_url)
        .await
        .expect("error creating database pool");
    let app = config::router::routes_all(&db_pool).await;
    let listener = tokio::net::TcpListener::bind(&"0.0.0.0:8080").await.unwrap();

    info!("LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, app).await.unwrap();
}
