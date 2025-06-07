// for now this module does not will be used
use uuid;
use serde::{ Serialize, Deserialize };
use chrono::{ DateTime, Utc };

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

impl Role {
    pub fn new(name: String, description: String) -> Self {
        Role {
            id: uuid::Uuid::new_v4(),
            description,
            name,
            created_at: Utc::now(),
        }
    }
}
