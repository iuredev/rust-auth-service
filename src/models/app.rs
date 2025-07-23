use redis::aio::ConnectionManager;
use sqlx::{ Pool, Postgres };

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub redis: ConnectionManager,
}
