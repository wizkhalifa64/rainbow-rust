use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct AgentDetails {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub photo: String,
    pub verified: bool,
    pub password: String,
    pub assign_to: Uuid,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}
