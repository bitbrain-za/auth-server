mod config;
mod handler;
mod jwt_auth;
mod response;
mod route;
mod user_model;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use axum::{response::IntoResponse, Json};
use config::Config;
use route::create_router;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub struct AppState {
    pub env: Config,
    pub db: Pool<MySql>,
}

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "JWT Authentication in Rust using Axum, Postgres, and SQLX";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

#[tokio::main]
async fn main() {
    //sqlx::migrate!().run(<&your_pool OR &mut your_connection>).await?;

    let config = Config::init();
    println!("{:?}", config);

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(config.db_url.as_str())
        .await
        .expect("Failed to connect to the DB");

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

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:4001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
