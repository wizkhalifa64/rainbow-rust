use std::sync::Arc;

use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;

use crate::{controller::usercontroller::register_handler, AppState};
pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Jwt auth";
    let json_message = json!({
        "status":"Success",
        "message":MESSAGE
    });
    Json(json_message)
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/test", get(health_checker_handler))
        .route("/api/auth/register", post(register_handler))
        .with_state(app_state)
}
