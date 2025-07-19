use axum::extract::{Json, State};

use crate::{
    db::{auth::upsert_refresh_token, user::get_user_by_email},
    errors::my_error::MyError,
    models::auth::{Login, TokenResponse},
    services::{auth::verify_password, jwt::generate_tokens},
};

pub async fn login_handler(
    State(pool): State<sqlx::Pool<sqlx::Postgres>>,
    Json(payload): Json<Login>,
) -> Result<Json<TokenResponse>, MyError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(MyError::Validation(
            "Email and password are required".to_string(),
        ));
    }

    let user = get_user_by_email(&pool, payload.email.clone()).await?;
    let verify_password = verify_password(&user.password, &payload.password).unwrap();

    if !verify_password {
        return Err(MyError::LoginError(
            "Invalid username or password".to_string(),
        ));
    }

    let (access_token, refresh_token) = generate_tokens(&user)?;

    let _ = upsert_refresh_token(&pool, user.id, &refresh_token).await;

    Ok(Json(TokenResponse {
        access_token,
        refresh_token,
    }))
}
