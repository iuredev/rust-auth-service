use serde::{Deserialize, Serialize};
use sqlx::Type;

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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "token_type", rename_all = "lowercase")]
pub enum TokenType {
    Access,
    Refresh,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Claims {
    pub sub: uuid::Uuid, // user_id
    pub email: String,
    pub roles: Vec<String>, // ["Admin", "User"]
    pub jti: String, //
    pub iat: usize, //
    pub exp: usize, //
    pub token_type: TokenType,
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

#[derive(Deserialize)]
pub struct RefreshTokenInput {
    pub refresh_token: String,
}
