use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use dotenv::dotenv;

pub type DbPool = Pool<Postgres>;

pub async fn init_db_pool() -> Result<DbPool, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}