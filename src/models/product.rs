use serde::{Deserialize, Serialize};
use sqlx::FromRow;
#[derive(Deserialize, Serialize, Debug, Clone, FromRow)]
pub struct Product {
    pub id: i32,
    pub lob: String,
    pub title: String,
    pub status: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, FromRow)]
pub struct SubProduct {
    pub id: i32,
    pub title: String,
    pub status: bool,
    pub alias: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProductInput {
    pub lob: String,
    pub title: String,
}
