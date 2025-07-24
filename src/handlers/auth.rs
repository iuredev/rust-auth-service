use axum::{
    Extension,
    extract::{Json, State},
};
use redis::AsyncCommands;

use crate::{
    auth::auth::generate_tokens,
    db::{
        auth::{revoke_refresh_token, upsert_refresh_token},
        user::get_user_by_email,
    },
    errors::my_error::MyError,
    models::{
        app::AppState,
        auth::{Claims, Login, TokenResponse},
    },
    services::password::verify_password,
};

pub async fn login_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<Login>,
) -> Result<Json<TokenResponse>, MyError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(MyError::Validation(
            "Email and password are required".to_string(),
        ));
    }

    let user = get_user_by_email(&app_state.pool, payload.email.clone()).await?;
    let verify_password = verify_password(&user.password, &payload.password).unwrap();

    if !verify_password {
        return Err(MyError::LoginError(
            "Invalid username or password".to_string(),
        ));
    }

    let (access_token, refresh_token) = generate_tokens(&user)?;

    let _ = upsert_refresh_token(&app_state.pool, user.id, &refresh_token).await;

    Ok(Json(TokenResponse {
        access_token,
        refresh_token,
    }))
}

// HANDLER USED WITH EXTRACTOR

// pub async fn logout_handler(
//     claims: Claims,
//     State(pool): State<sqlx::Pool<sqlx::Postgres>>
// ) -> Result<Json<serde_json::Value>, MyError> {
//     let _ = revoke_refresh_token(&pool, claims.sub).await;

//     Ok(Json(serde_json::json!({
//         "message": "Logged out successfully",
//     })))
// }

// HANDLER USED WITH MIDDLEWARE AUTH
pub async fn logout_handler(
    Extension(claims): Extension<Claims>,
    State(app_state): State<AppState>,
) -> Result<Json<serde_json::Value>, MyError> {
    let _ = revoke_refresh_token(&app_state.pool, claims.sub).await;

    let mut redis_conn = app_state.redis;
    let key_jti = format!("jti_revoked:{}", claims.jti);
    let _: bool = redis_conn
        .set_ex(key_jti, true, 600)
        .await
        .map_err(|_| MyError::Internal)?;

    Ok(Json(serde_json::json!({
        "message": "Logged out successfully",
    })))
}
