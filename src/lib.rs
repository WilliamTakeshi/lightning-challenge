use std::time::Duration;

use axum::{
    extract::State, http::StatusCode, response::IntoResponse, routing::get, serve::Serve, Json,
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Execute, PgPool, Pool, Postgres, QueryBuilder};
use tokio::net::TcpListener;

pub fn run(listener: TcpListener, pool: Pool<Postgres>) -> Serve<Router, Router> {
    tracing_subscriber::fmt::init();

    // sqlx::migrate!().run(<&your_pool OR &mut your_connection>).await?;
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/load_nodes", get(load_nodes))
        .route("/nodes", get(nodes))
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

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

#[derive(Debug, Deserialize, Serialize)]
struct Node {
    #[serde(alias = "publicKey")]
    public_key: String,
    alias: String,
    capacity: i64,
    #[serde(alias = "firstSeen")]
    first_seen: i64,
}

async fn nodes(State(pool): State<PgPool>) -> Result<Json<Vec<Node>>, (StatusCode, String)> {
    let nodes = sqlx::query_as!(
        Node,
        r#"
        SELECT public_key, alias, capacity, first_seen
        FROM nodes
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(not_found)?;

    Ok(Json(nodes))
}

async fn load_nodes(State(pool): State<PgPool>) -> Result<(), (StatusCode, String)> {
    // URL of the API endpoint providing the nodes data
    // Should this be hardcoded?
    let url = "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

    let response = match reqwest::get(url).await {
        Ok(response) => response,
        Err(error) => {
            tracing::error!("Failed to send request: {:?}", error);
            return Err(internal_error());
        }
    };

    let nodes: Vec<Node> = match response.json().await {
        Ok(nodes) => nodes,
        Err(error) => {
            tracing::error!("Failed to parse response body as JSON: {:?}", error);
            return Err(internal_error());
        }
    };

    dbg!(&nodes);

    // TODO: there is a bind limit on postgres, failing to respect this value can cause a panic
    // FIX ME: use a batch insert
    let mut query_builder =
        QueryBuilder::new("INSERT INTO nodes (public_key, alias, capacity, first_seen) ");

    query_builder.push_values(nodes, |mut b, node| {
        b.push_bind(node.public_key)
            .push_bind(node.alias)
            .push_bind(node.capacity)
            .push_bind(node.first_seen);
    });

    // TODO: ADD on conflict

    let query = query_builder.build();

    dbg!(query.sql());
    query.execute(&pool).await.map_err(unprocessable_entity)?;

    // dbg!(wallet);

    Ok(())
}

fn internal_error() -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal Server Error".to_string(),
    )
}

fn unprocessable_entity<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::UNPROCESSABLE_ENTITY, err.to_string())
}

fn not_found<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::NOT_FOUND, err.to_string())
}
