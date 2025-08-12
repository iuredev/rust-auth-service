use crate::{
    db::role::{get_default_role, set_user_role}, errors::my_error::MyError, models::user::{User, UserOutput, UserRegister, UserWithRoles}
};
use chrono::Utc;
use sqlx::{Pool, Postgres};

pub async fn create_user(pool: &Pool<Postgres>, user: UserRegister) -> Result<UserOutput, MyError> {
    let user = User::new(
        user.name.unwrap_or_default(),
        user.email.unwrap_or_default(),
        user.password.unwrap_or_default(),
    );

    let result = sqlx::query_as!(
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
    .await?;

    let get_default_role = get_default_role(pool).await?;
    let _ = set_user_role(pool, user.id, get_default_role.id).await?;

    Ok(result)
}

pub async fn get_user_by_id(pool: &Pool<Postgres>, id: uuid::Uuid) -> Result<UserWithRoles, MyError> {
    let user = sqlx::query_as!(
            UserWithRoles,
            r#"
            SELECT
                u.id,
                u.name,
                u.email,
                u.created_at,
                u.updated_at,
                COALESCE(ARRAY_AGG(r.name) FILTER (WHERE r.name IS NOT NULL), '{}') AS "roles!"
            FROM users u
            LEFT JOIN user_roles ur ON ur.user_id = u.id
            LEFT JOIN roles r ON r.id = ur.role_id
            WHERE u.id = $1
            GROUP BY u.id
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(|_| MyError::NotFound)?;

    Ok(user)
}

pub async fn get_users(pool: &Pool<Postgres>) -> Result<Vec<UserOutput>, MyError> {
    let users =
        sqlx::query_as::<_, UserOutput>("SELECT id, name, email, created_at, updated_at FROM users")
            .fetch_all(pool)
            .await?;

    Ok(users)
}

pub async fn update_user(
    pool: &Pool<Postgres>,
    id: uuid::Uuid,
    user: UserRegister,
) -> Result<UserOutput, MyError> {
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
    .await;

    let user = match user {
        Ok(u) => u,
        Err(sqlx::Error::RowNotFound) => return Err(MyError::NotFound),
        Err(e) => return Err(MyError::DatabaseError(e)),
    };

    Ok(user)
}

pub async fn delete_user(pool: &Pool<Postgres>, id: uuid::Uuid) -> Result<(), MyError> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_user_by_email(
    pool: &Pool<Postgres>,
    email: String,
) -> Result<Option<User>, MyError> {
    let email = email.trim().to_lowercase();

    let user = sqlx::query_as::<_, User>(
        "SELECT id, name, email, password, created_at, updated_at FROM users WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}
