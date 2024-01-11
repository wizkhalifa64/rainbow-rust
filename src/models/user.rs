use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub mobile: Option<String>,
    pub password: String,
    pub role: Option<i16>,
    pub photo: String,
    pub verified: bool,
    pub address: Option<Value>,
    pub area_list: Option<Value>,
    pub qualification: Option<Value>,
    pub employee_id: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: Option<i16>,
    pub address: Option<Value>,
    pub area_list: Option<Value>,
    pub qualification: Option<Value>,
    pub employee_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}
