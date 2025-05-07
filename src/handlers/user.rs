use crate::config::init_pool;
use crate::db::user::{create_user, delete_user, get_user_by_id, update_user};
use crate::errors::my_error::MyError;
use crate::models::user::{UserInput, UserOutput};
use crate::utils::password::hash_password;
use axum::extract::{Json, Path};

pub async fn get_user_handler(
    Path(user_id): Path<uuid::Uuid>,
) -> Result<Json<UserOutput>, MyError> {
    let pool: sqlx::Pool<sqlx::Postgres> = init_pool().await;

    let user = get_user_by_id(&pool, user_id).await?;

    Ok(Json(user))
}

pub async fn create_user_handler(
    Json(payload): Json<UserInput>,
) -> Result<Json<UserOutput>, MyError> {
    let pool: sqlx::Pool<sqlx::Postgres> = init_pool().await;

    if payload.name.is_none() || payload.email.is_none() || payload.password.is_none() {
        return Err(MyError::Validation(
            "Name, email and password are required".to_string(),
        ));
    }

    let user = UserInput {
        name: payload.name,
        email: payload.email,
        password: payload.password,
    };

    let result = create_user(&pool, user).await?;

    Ok(Json(result))
}

pub async fn update_user_handler(
    Path(user_id): Path<uuid::Uuid>,
    Json(mut payload): Json<UserInput>,
) -> Result<Json<UserOutput>, MyError> {
    let pool: sqlx::Pool<sqlx::Postgres> = init_pool().await;

    if payload.name.is_none() && payload.email.is_none() && payload.password.is_none() {
        return Err(MyError::Validation(
            "You must provide at least one field to update".to_string(),
        ));
    }

    if payload.password.is_some() {
        let hash_password = hash_password(&payload.password.unwrap()).unwrap();
        payload.password = Some(hash_password);
    }

    let user = UserInput {
        name: payload.name,
        email: payload.email,
        password: payload.password,
    };

    let result = update_user(&pool, user_id, user).await?;

    Ok(Json(result))
}

pub async fn delete_user_handler(Path(user_id): Path<uuid::Uuid>) -> Result<Json<String>, MyError> {
    let pool: sqlx::Pool<sqlx::Postgres> = init_pool().await;

    delete_user(&pool, user_id).await?;

    Ok(Json("User deleted successfully".to_string()))
}
