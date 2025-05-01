use sqlx::PgPool;
use std::env;

pub async fn init_pool() -> PgPool {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to the database")
}
