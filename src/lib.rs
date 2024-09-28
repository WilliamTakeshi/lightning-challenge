use axum::{http::StatusCode, response::IntoResponse, routing::get, serve::Serve, Router};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

pub fn run(listener: TcpListener) -> Serve<Router, Router> {
    tracing_subscriber::fmt::init();

    // sqlx::migrate!().run(<&your_pool OR &mut your_connection>).await?;
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/load_nodes", get(load_nodes));

    axum::serve(listener, app)
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

#[derive(Debug, Deserialize, Serialize)]
struct Node {
    #[serde(alias = "publicKey")]
    public_key: String,
    alias: String,
    capacity: u64,
    #[serde(alias = "firstSeen")]
    first_seen: u64,
}

async fn load_nodes() -> impl IntoResponse {
    // URL of the API endpoint providing the nodes data
    // Should this be hardcoded?
    let url = "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

    let response = match reqwest::get(url).await {
        Ok(response) => response,
        Err(error) => {
            tracing::error!("Failed to send request: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR, ());
        }
    };


    let nodes: Vec<Node> = match response.json().await {
        Ok(nodes) => nodes,
        Err(error) => {
            tracing::error!("Failed to parse response body as JSON: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR, ());
        }
    };

    (StatusCode::OK, ())
}
