use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

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

    let db_url = std::env::var("DATABASE_URL")
        .expect("missing environment variable `DATABASE_URL`")
        .to_owned();
    // let db_pool = SqlitePool::connect(db_url.as_str()).await.unwrap();
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("error creating database pool");
    let app = config::router::routes_all(&db_pool).await;
    let listener = tokio::net::TcpListener::bind(&"0.0.0.0:8080").await.unwrap();

    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, app).await.unwrap();
}
