use std::sync::Arc;

use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;

use crate::{
    controller::{
        productcontroller::create_product,
        usercontroller::{login_handler, register_handler},
    },
    AppState,
};
pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Jwt auth";
    let json_message = json!({
        "status":"Success",
        "message":MESSAGE
    });
    Json(json_message)
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let auth_route = Router::new()
        .route("/test", get(health_checker_handler))
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/create-product", post(create_product));
    Router::new()
        .nest("/auth", auth_route)
        .with_state(app_state)
}
