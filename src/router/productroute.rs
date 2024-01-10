use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    controller::productcontroller::{
        create_product, create_subproduct, get_product, get_product_by_id, get_sub_product,
    },
    AppState,
};

pub fn product_router(app_state: Arc<AppState>) -> Router {
    let product_route = Router::new()
        .route("/create-product", post(create_product))
        .route("/create-subproduct", post(create_subproduct))
        .route("/get-products", get(get_product))
        .route("/get-products-by-id/:product_id", post(get_product_by_id))
        .route("/get-all-subproducts", get(get_sub_product))
        .with_state(app_state);
    product_route
}
