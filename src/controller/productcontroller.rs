use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

use crate::{
    models::product::{Product, ProductInput},
    AppState,
};

pub async fn create_product(
    State(state): State<Arc<AppState>>,
    Json(body): Json<ProductInput>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let saved_product = sqlx::query_as!(
        Product,
        "INSERT INTO products (lob,title) VALUES ($1,$2) RETURNING *",
        body.lob.to_string(),
        body.title.to_string()
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;
    let product_response = json!({"status": "success","data": saved_product});
    Ok(Json(product_response))
}
