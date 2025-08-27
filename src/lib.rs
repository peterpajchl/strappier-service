use anyhow::Result;
use axum::{Router, routing::get};
use std::env::var;
use tokio::net::TcpListener;

pub async fn run() -> Result<()> {
    let app_port = var("APP_PORT").unwrap_or("3070".to_string());

    let tcp_listener = TcpListener::bind(format!("0.0.0.0:{}", app_port)).await?;

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    axum::serve(tcp_listener, app).await?;

    Ok(())
}
