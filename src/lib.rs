use std::time::Duration;

use axum::{routing::get, serve::Serve, Router};
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

pub mod errors;
pub mod routes;

pub fn run(listener: TcpListener, pool: Pool<Postgres>) -> Serve<Router, Router> {
    let app = Router::new()
        .route("/health_check", get(routes::health_check::health_check))
        .route("/load_nodes", get(routes::nodes::load_nodes_endpoint))
        .route("/nodes", get(routes::nodes::nodes))
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    axum::serve(listener, app)
}

pub async fn setup_db() -> PgPool {
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://app:password@localhost:5432/lightning".to_string());

    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database")
}
