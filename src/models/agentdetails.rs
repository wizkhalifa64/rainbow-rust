use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct AgentDetails {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub photo: String,
    pub verified: bool,
    pub password: String,
    pub assign_to: Option<i32>,
    pub agent_code: Option<String>,
    pub state: Option<i32>,
    pub city: Option<i32>,
    pub status: Option<i16>,
    pub qualification: Option<Value>,
    pub working_area: Option<Value>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}
pub struct AgentRegisterIput {
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub photo: String,
    pub password: String,
    pub state: Option<i32>,
    pub city: Option<i32>,
    pub qualification: Option<Value>,
}
