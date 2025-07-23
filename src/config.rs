use redis::aio::ConnectionManager;
use sqlx::PgPool;
use std::env;

pub async fn init_pool() -> PgPool {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to the database")
}

pub async fn init_redis() -> ConnectionManager {
    dotenvy::dotenv().ok();

    let redis = env::var("REDIS_URL").expect("REDIS_URL must be set");

    let client = redis::Client::open(redis).unwrap();
    ConnectionManager::new(client)
        .await
        .expect("Failed to connect to Redis")
}
