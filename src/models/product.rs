use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use validator::Validate;
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
    #[serde(rename = "productId")]
    pub product_id: Option<i32>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct ProductInput {
    #[validate(length(min = 1))]
    pub lob: String,
    #[validate(length(min = 1))]
    pub title: String,
}
#[derive(Deserialize, Debug, Validate)]
pub struct SubProductInput {
    #[validate(length(min = 1))]
    pub alias: String,
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(range(min = 0))]
    pub product_id: i32,
}

#[derive(Serialize, Debug)]
pub struct GetProduct {
    pub id: i32,
    pub lob: String,
    pub title: String,
    pub status: bool,
    pub subproduct: Value,
}
#[derive(Deserialize, Debug, Validate)]
pub struct GetProductCriteriaInput {
    #[validate(range(min = 0))]
    pub page_no: i8,
    #[validate(range(min = 0))]
    pub page_size: i16,
    #[validate(range(min = 0))]
    pub id: Option<i32>,
}
