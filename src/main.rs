mod config;
use axum::{response::IntoResponse, routing::get, Json, Router};
use config::Config;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::sync::Arc;

pub struct AppState {
    pub config: Config,
    pub pool: Pool<MySql>,
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

    let app = Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .with_state(Arc::new(AppState {
            config: config.clone(),
            pool: pool.clone(),
        }));

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:4001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
