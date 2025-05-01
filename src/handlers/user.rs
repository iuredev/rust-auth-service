use crate::config::init_pool;
use crate::db::user::{create_user, delete_user, get_user_by_id, update_user};
use crate::models::user::{UserInput, UserOutput};
use axum::extract::{Json, Path};
use axum::http::StatusCode;

pub async fn get_user_handler(
    Path(user_id): Path<uuid::Uuid>,
) -> Result<Json<UserOutput>, StatusCode> {
    let pool: sqlx::Pool<sqlx::Postgres> = init_pool().await;

    let user = get_user_by_id(&pool, user_id).await;

    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_user_handler(
    Json(payload): Json<UserInput>,
) -> Result<Json<UserOutput>, StatusCode> {
    let pool: sqlx::Pool<sqlx::Postgres> = init_pool().await;

    if payload.name.is_none() || payload.email.is_none() || payload.password.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let user = UserInput {
        name: payload.name,
        email: payload.email,
        password: payload.password,
    };

    let result = create_user(&pool, user).await;

    match result {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_user_handler(
    Path(user_id): Path<uuid::Uuid>,
    Json(payload): Json<UserInput>,
) -> Result<Json<UserOutput>, StatusCode> {
    let pool: sqlx::Pool<sqlx::Postgres> = init_pool().await;

    if payload.name.is_none() && payload.email.is_none() && payload.password.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let user = UserInput {
        name: payload.name,
        email: payload.email,
        password: payload.password,
    };

    let result = update_user(&pool, user_id, user).await;

    match result {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_user_handler(
    Path(user_id): Path<uuid::Uuid>,
) -> Result<Json<String>, StatusCode> {
    let pool: sqlx::Pool<sqlx::Postgres> = init_pool().await;

    let result = delete_user(&pool, user_id).await;

    match result {
        Ok(()) => Ok(Json("User deleted successfully".to_string())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
