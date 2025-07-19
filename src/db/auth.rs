use sqlx::{Pool, Postgres};

use crate::{errors::my_error::MyError, models::auth::RefreshToken};

pub async fn create_refresh_token(
    pool: &Pool<Postgres>,
    user_id: uuid::Uuid,
    refresh_token: &String,
) -> Result<(), MyError> {
    let refresh_token = RefreshToken::new(refresh_token.to_string(), user_id);

    let result = sqlx::query(
        r#"
            INSERT INTO refresh_tokens (id, token, user_id, expires_at)
            VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(refresh_token.id)
    .bind(refresh_token.token)
    .bind(refresh_token.user_id)
    .bind(refresh_token.expires_at)
    .execute(pool)
    .await;

    if result.is_err() {
        return Err(MyError::DatabaseError(result.unwrap_err()));
    }

    Ok(())
}

pub async fn delete_refresh_token(pool: &Pool<Postgres>, token: String) -> Result<(), MyError> {
    let result = sqlx::query("DELETE FROM refresh_tokens WHERE token = $1")
        .bind(token)
        .execute(pool)
        .await;

    if result.is_err() {
        return Err(MyError::DatabaseError(result.unwrap_err()));
    }

    Ok(())
}
