use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{errors::my_error::MyError, models::role::Role};

pub async fn get_user_roles(pool: &Pool<Postgres>, user_id: Uuid) -> Result<Vec<String>, MyError> {

    let user_roles = sqlx::query_scalar!(r#"SELECT r.name FROM roles r INNER JOIN user_roles ur ON ur.role_id = r.id WHERE ur.user_id = $1"#, user_id).fetch_all(pool).await?;

    println!("User Roles, {:#?}", user_roles);

    Ok(user_roles)
}

pub async fn get_default_role(pool: &Pool<Postgres>) -> Result<Role, MyError> {
        let default_role = sqlx::query_as!(Role, r#"SELECT * FROM roles WHERE name = 'User'"#).fetch_one(pool).await?;

        Ok(default_role)
    }

pub async fn set_user_role(
        pool: &Pool<Postgres>,
        user_id: Uuid,
        role_id: Uuid
    ) -> Result<(), MyError> {
        sqlx::query!(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            VALUES ($1, $2)
            "#,
            user_id,
            role_id
        )
        .execute(pool)
        .await
        .map_err(|err| MyError::DatabaseError(err))?;

        Ok(())
    }
