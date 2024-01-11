use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

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
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterUserSchema {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_list: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginUserSchema {
    #[validate(length(min = 1))]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}
