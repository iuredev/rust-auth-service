use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::password::hash_password;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInput {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserOutput {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(name: String, email: String, password: String) -> Self {
        let name = name.trim().to_string();
        let email = email.trim().to_string();
        let password = password.trim().to_string();

        Self {
            id: uuid::Uuid::new_v4(),
            name,
            email,
            password: hash_password(password.as_str()).unwrap(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
