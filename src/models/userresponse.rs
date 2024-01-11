use chrono::prelude::*;
use serde::Serialize;
use serde_json::Value;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub photo: String,
    pub role: Option<i16>,
    pub verified: bool,
    pub areaList: Option<Value>,
    #[serde(rename = "createdAt")]
    pub createdAt: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updatedAt: DateTime<Utc>,
}
