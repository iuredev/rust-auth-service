use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: uuid::Uuid,
    pub email: String,
    // pub role: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct RefreshToken {
    pub id: uuid::Uuid,
    pub token: String,
    pub user_id: uuid::Uuid,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

impl RefreshToken {
    pub fn new(token: String, user_id: uuid::Uuid) -> Self {
        RefreshToken {
            id: uuid::Uuid::new_v4(),
            token,
            user_id,
            expires_at: chrono::Utc::now() + chrono::Duration::days(7),
        }
    }
}
