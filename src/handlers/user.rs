use crate::db::user::{create_user, delete_user, get_user_by_email, get_user_by_id, update_user};
use crate::errors::my_error::MyError;

use crate::models::user::UserWithRoles;
use crate::models::{
    app::AppState,
    user::{UserOutput, UserRegister},
};
use crate::services::password::hash_password;
use axum::extract::{Json, Path, State};

#[utoipa::path(
    get,
    path = "/api/users/{user_id}",
    params(
        ("user_id" = uuid::Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User retrieved successfully", body = UserWithRoles),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "users"
)]
pub async fn get_user_handler(
    State(app_state): State<AppState>,
    Path(user_id): Path<uuid::Uuid>,
) -> Result<Json<UserWithRoles>, MyError> {
    let user = get_user_by_id(&app_state.pool, user_id).await?;

    Ok(Json(user))
}

#[utoipa::path(
    post,
    path = "/api/users",
    request_body = UserRegister,
    responses(
        (status = 200, description = "User created successfully", body = UserOutput),
        (status = 400, description = "Validation error"),
        (status = 409, description = "User already exists"),
    ),
    tag = "users"
)]
pub async fn create_user_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<UserRegister>,
) -> Result<Json<UserOutput>, MyError> {
    if payload.name.is_none() || payload.email.is_none() || payload.password.is_none() {
        return Err(MyError::Validation(
            "Name, email and password are required".to_string(),
        ));
    }

    let exist_user =
        get_user_by_email(&app_state.pool, Option::expect(payload.email.clone(), "")).await?;

    if exist_user.is_some() {
        return Err(MyError::Validation("User already exists".to_string()));
    }

    let user: UserRegister = UserRegister {
        name: payload.name,
        email: payload.email,
        password: payload.password,
    };

    let result = create_user(&app_state.pool, user).await?;



    Ok(Json(result))
}

#[utoipa::path(
    patch,
    path = "/api/users/{user_id}",
    params(
        ("user_id" = uuid::Uuid, Path, description = "User ID")
    ),
    request_body = UserRegister,
    responses(
        (status = 200, description = "User updated successfully", body = UserOutput),
        (status = 400, description = "Validation error"),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "users"
)]
pub async fn update_user_handler(
    State(app_state): State<AppState>,
    Path(user_id): Path<uuid::Uuid>,
    Json(mut payload): Json<UserRegister>,
) -> Result<Json<UserOutput>, MyError> {
    if payload.name.is_none() && payload.email.is_none() && payload.password.is_none() {
        return Err(MyError::Validation(
            "You must provide at least one field to update".to_string(),
        ));
    }

    if payload.password.is_some() {
        let hash_password = hash_password(&payload.password.unwrap()).unwrap();
        payload.password = Some(hash_password);
    }

    let user = UserRegister {
        name: payload.name,
        email: Some(payload.email.unwrap().to_lowercase()),
        password: payload.password,
    };

    let result = update_user(&app_state.pool, user_id, user).await?;

    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "/api/users/{user_id}",
    params(
        ("user_id" = uuid::Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User deleted successfully", body = String),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "users"
)]
pub async fn delete_user_handler(
    State(app_state): State<AppState>,
    Path(user_id): Path<uuid::Uuid>,
) -> Result<Json<String>, MyError> {
    delete_user(&app_state.pool, user_id).await?;

    Ok(Json("User deleted successfully".to_string()))
}
