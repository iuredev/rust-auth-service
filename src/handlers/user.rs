use crate::db::user::{create_user, delete_user, get_user_by_email, get_user_by_id, update_user};
use crate::errors::my_error::MyError;
use crate::models::{
    app::AppState,
    user::{UserOutput, UserRegister},
};
use crate::services::password::hash_password;
use axum::extract::{Json, Path, State};

pub async fn get_user_handler(
    State(app_state): State<AppState>,
    Path(user_id): Path<uuid::Uuid>,
) -> Result<Json<UserOutput>, MyError> {
    let user = get_user_by_id(&app_state.pool, user_id).await?;

    Ok(Json(user))
}

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

pub async fn delete_user_handler(
    State(app_state): State<AppState>,
    Path(user_id): Path<uuid::Uuid>,
) -> Result<Json<String>, MyError> {
    delete_user(&app_state.pool, user_id).await?;

    Ok(Json("User deleted successfully".to_string()))
}
