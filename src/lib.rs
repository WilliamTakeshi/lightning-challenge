use axum::{http::StatusCode, response::IntoResponse, routing::get, serve::Serve, Router};
use tokio::net::TcpListener;

pub fn run(listener: TcpListener) -> Serve<Router, Router> {
    tracing_subscriber::fmt::init();
    
    let app = Router::new().route("/health_check", get(health_check));
    axum::serve(listener, app)
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}
