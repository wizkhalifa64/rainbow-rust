use std::sync::Arc;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    response::IntoResponse,
    Json, Server,
};
use dotenv::dotenv;
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::cors::CorsLayer;

use crate::{config::config::Config, router::auth::create_router};
pub mod config;
pub mod controller;
pub mod middleware;
pub mod models;
pub mod router;
#[allow(unused)]
pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::init();
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("databasse connected");
            pool
        }
        Err(err) => {
            println!("Database connected error ,{:?}", err);
            std::process::exit(1);
        }
    };
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState {
        db: pool.clone(),
        env: config.clone(),
    }))
    .layer(cors);

    println!("🚀 server started successfuly");
    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Jwt auth";
    let json_message = json!({
        "status":"Success",
        "message":MESSAGE
    });
    Json(json_message)
}
