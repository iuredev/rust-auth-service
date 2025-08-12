use uuid::{self, Uuid};
use serde::{ Serialize, Deserialize };
use chrono::{ DateTime, Utc };
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Role {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserRole {
    pub user_id: Uuid,
    pub role_id: Uuid
}

impl Role {
    pub fn new(name: String, description: Option<String>) -> Self {
        let now =  Utc::now();

        Role {
            id: uuid::Uuid::new_v4(),
            description,
            name,
            created_at: now,
            updated_at: now
        }
    }
}
