use sqlx::{ Pool, Postgres };

use crate::{ errors::my_error::MyError, models::auth::RefreshToken };

pub async fn upsert_refresh_token(
    pool: &Pool<Postgres>,
    user_id: uuid::Uuid,
    refresh_token: &String
) -> Result<(), MyError> {
    let refresh_token = RefreshToken::new(refresh_token.to_string(), user_id);

    let result = sqlx
        ::query(
            r#"
        INSERT INTO refresh_tokens (id, token, user_id, expires_at)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (user_id)
        DO UPDATE SET
            token = EXCLUDED.token,
            expires_at = EXCLUDED.expires_at
    "#
        )
        .bind(refresh_token.id)
        .bind(refresh_token.token)
        .bind(refresh_token.user_id)
        .bind(refresh_token.expires_at)
        .execute(pool).await;

    if result.is_err() {
        return Err(MyError::DatabaseError(result.unwrap_err()));
    }

    Ok(())
}

pub async fn revoke_refresh_token(
    pool: &Pool<Postgres>,
    user_id: uuid::Uuid
) -> Result<(), MyError> {
    let result = sqlx
        ::query("DELETE FROM refresh_tokens WHERE user_id = $1")
        .bind(user_id)
        .execute(pool).await;

    if result.is_err() {
        return Err(MyError::DatabaseError(result.unwrap_err()));
    }

    Ok(())
}
