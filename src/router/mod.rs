use std::sync::Arc;

use axum::Router;

use crate::AppState;

use self::{auth::auth_router, productroute::product_router};

pub mod auth;
pub mod productroute;
pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/auth", auth_router(app_state.clone()))
        .nest("/products", product_router(app_state.clone()))
}
