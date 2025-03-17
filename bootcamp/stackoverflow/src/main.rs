mod models;
use dotenvy::dotenv;
use log::*;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to create Postgres connection pool");

    let recs = sqlx::query!("SELECT * FROM questions")
        .fetch_all(&pool)
        .await
        .unwrap();
}
