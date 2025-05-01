use crate::models::user::{User, UserInput, UserOutput};
use chrono::Utc;
use sqlx::{Pool, Postgres};

pub async fn create_user(
    pool: &Pool<Postgres>,
    user: UserInput,
) -> Result<UserOutput, sqlx::Error> {
    let user = User::new(
        user.name.unwrap_or_default(),
        user.email.unwrap_or_default(),
        user.password.unwrap_or_default(),
    );

    sqlx::query_as!(
        UserOutput,
        r#"
        INSERT INTO users (id, name, email, password, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, name, email, created_at, updated_at
        "#,
        user.id,
        user.name,
        user.email,
        user.password,
        user.created_at,
        user.updated_at
    )
    .fetch_one(pool)
    .await
}

pub async fn get_user_by_id(
    pool: &Pool<Postgres>,
    id: uuid::Uuid,
) -> Result<UserOutput, sqlx::Error> {
    println!("ID: {}", id);

    let user: UserOutput = sqlx::query_as::<_, UserOutput>(
        "SELECT id, name, email, created_at, updated_at FROM users WHERE id = $1",
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn update_user(
    pool: &Pool<Postgres>,
    id: uuid::Uuid,
    user: UserInput,
) -> Result<UserOutput, sqlx::Error> {
    let user = sqlx::query_as::<_, UserOutput>(
        r#"
            UPDATE users SET name = COALESCE($1, name), 
            email = COALESCE($2, email), 
            password = COALESCE($3, password),
            updated_at = COALESCE($5, updated_at)
            WHERE id = $4 RETURNING id, name, email, created_at, updated_at
        "#,
    )
    .bind(user.name)
    .bind(user.email)
    .bind(user.password)
    .bind(id)
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn delete_user(pool: &Pool<Postgres>, id: uuid::Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
