use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::{json, Value};

use crate::{
    middleware::validator::ValidatedJson,
    models::product::{GetProduct, Product, ProductInput, SubProduct, SubProductInput},
    AppState,
};

pub async fn create_product(
    State(state): State<Arc<AppState>>,
    ValidatedJson(body): ValidatedJson<ProductInput>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let saved_product = sqlx::query_as!(
        Product,
        "INSERT INTO tbl_products (lob,title) VALUES ($1,$2) RETURNING *",
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

pub async fn create_subproduct(
    State(state): State<Arc<AppState>>,
    ValidatedJson(body): ValidatedJson<SubProductInput>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let check_foreign_key: Option<bool> =
        sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM products WHERE id = $1)")
            .bind(body.product_id)
            .fetch_one(&state.db)
            .await
            .map_err(|e| {
                let error_response = json!({
                    "status": "fail",
                    "message": format!("Database error: {}", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;
    if let Some(exists) = check_foreign_key {
        if !exists {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "Product not exist",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
    }
    let save_subproduct = sqlx::query_as!(
        SubProduct,
        "INSERT INTO tbl_subproducts (title,product_id,alias) VALUES ($1,$2,$3) RETURNING *",
        body.title.to_string(),
        body.product_id,
        body.alias.to_string()
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
    let product_response = json!({"status": "success","data": save_subproduct});
    Ok(Json(product_response))
}

pub async fn get_product(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let products = sqlx::query_as!(
        GetProduct,
        r#"
        SELECT p.id, 
        p.title,
        p.lob,
        p.status,
        COALESCE(jsonb_agg(sp) FILTER (WHERE sp.id IS NOT NULL),'[]') AS subproduct
    FROM tbl_products p
    LEFT JOIN tbl_subproducts sp ON p.id = sp.product_id
    GROUP BY p.id
    "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    Ok(Json(json!({"data":products})))
}

pub async fn get_product_by_id(
    State(state): State<Arc<AppState>>,
    Path(product_id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let products = sqlx::query_as!(
        GetProduct,
        r#"
        SELECT p.id, 
        p.title,
        p.lob,
        p.status,
        COALESCE(jsonb_agg(sp) FILTER (WHERE sp.id IS NOT NULL),'[]') AS subproduct
    FROM tbl_products p
    LEFT JOIN tbl_subproducts sp ON p.id = sp.product_id
    WHERE p.id = $1
    GROUP BY p.id
    "#,
        product_id
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

    Ok(Json(json!({"data":products})))
}

pub async fn get_sub_product(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let subproducts = sqlx::query_as!(
        SubProduct,
        r#"
    SELECT * FROM tbl_subproducts
    "#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;
    Ok(Json(json!({"data":subproducts})))
}
