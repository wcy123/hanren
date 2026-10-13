//! HANREN inference server (axum + tokio)

use axum::{
    extract::Json,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Deserialize)]
struct GenerateRequest {
    prompt: String,
    max_tokens: Option<u32>,
}

#[derive(Debug, Serialize)]
struct GenerateResponse {
    text: String,
    tokens: u32,
}

async fn health() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "version": hanren::VERSION,
    }))
}

async fn generate(Json(req): Json<GenerateRequest>) -> impl IntoResponse {
    log::info!("Generate request: prompt={:?}", req.prompt);

    // TODO: actual inference
    let response = GenerateResponse {
        text: format!("[TODO] Generated response for: {}", req.prompt),
        tokens: req.max_tokens.unwrap_or(100),
    };

    Json(response)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let app = Router::new()
        .route("/health", get(health))
        .route("/v1/generate", post(generate));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    log::info!("🗡️  HANREN Server v{}", hanren::VERSION);
    log::info!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
