use axum::extract::{ Json, State };

use crate::{
    auth::jwt::generate_tokens,
    db::{ auth::{ revoke_refresh_token, upsert_refresh_token }, user::get_user_by_email },
    errors::my_error::MyError,
    models::auth::{ Login, TokenResponse, Claims },
    services::password::verify_password,
};

pub async fn login_handler(
    State(pool): State<sqlx::Pool<sqlx::Postgres>>,
    Json(payload): Json<Login>
) -> Result<Json<TokenResponse>, MyError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(MyError::Validation("Email and password are required".to_string()));
    }

    let user = get_user_by_email(&pool, payload.email.clone()).await?;
    let verify_password = verify_password(&user.password, &payload.password).unwrap();

    if !verify_password {
        return Err(MyError::LoginError("Invalid username or password".to_string()));
    }

    let (access_token, refresh_token) = generate_tokens(&user)?;

    let _ = upsert_refresh_token(&pool, user.id, &refresh_token).await;

    Ok(
        Json(TokenResponse {
            access_token,
            refresh_token,
        })
    )
}

pub async fn logout_handler(
    claims: Claims,
    State(pool): State<sqlx::Pool<sqlx::Postgres>>
) -> Result<Json<serde_json::Value>, MyError> {
    let _ = revoke_refresh_token(&pool, claims.sub).await;

    Ok(Json(serde_json::json!({
        "message": "Logged out successfully",
    })))
}
