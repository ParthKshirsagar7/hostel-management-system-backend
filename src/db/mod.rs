use std::env;
use dotenvy::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn establish_connection() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    println!("Connecting to databse...");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres")
}