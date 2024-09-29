use crate::errors::{internal_error, not_found};

use axum::{extract::State, http::StatusCode, Json};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Pool, Postgres, QueryBuilder};
const SATS_TO_BTC: i64 = 100000000;

#[derive(Debug, Deserialize, Serialize)]
struct Node {
    #[serde(alias = "publicKey")]
    public_key: String,
    alias: String,
    capacity: i64,
    #[serde(alias = "firstSeen")]
    first_seen: i64,
}

#[derive(Debug, Serialize)]
pub struct PrettyNode {
    public_key: String,
    alias: String,
    capacity: String,
    first_seen: String,
}

impl TryFrom<Node> for PrettyNode {
    // TODO: Better error for the conversion
    type Error = ();

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let Some(datetime) = DateTime::from_timestamp(node.first_seen, 0) else {
            return Err(());
        };

        Ok(PrettyNode {
            public_key: node.public_key,
            alias: node.alias,
            capacity: (node.capacity as f64 / SATS_TO_BTC as f64).to_string(), // TODO: Check if this is safe for large numbers
            first_seen: datetime.to_string(),
        })
    }
}

pub async fn nodes(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<PrettyNode>>, (StatusCode, String)> {
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

    // Here we are ignoring the nodes that failed to convert, should we return an error?
    let pretty_nodes: Vec<PrettyNode> = nodes
        .into_iter()
        .map(PrettyNode::try_from)
        .filter(|node| node.is_ok())
        .flatten()
        .collect();

    Ok(Json(pretty_nodes))
}

pub async fn load_nodes(pool: &Pool<Postgres>) -> Result<(), String> {
    // URL of the API endpoint providing the nodes data
    // Should this be hardcoded?
    // TODO: Fix hardcoded URL
    let url = "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

    let response = match reqwest::get(url).await {
        Ok(response) => response,
        Err(error) => {
            tracing::error!("Failed to send request: {:?}", error);
            return Err("Failed to send request".to_string());
        }
    };

    let nodes: Vec<Node> = match response.json().await {
        Ok(nodes) => nodes,
        Err(error) => {
            tracing::error!("Failed to parse response body as JSON: {:?}", error);
            return Err("Failed to parse response body as JSON".to_string());
        }
    };

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

    query_builder.push(
        " ON CONFLICT (public_key) DO UPDATE SET alias = EXCLUDED.alias,
    capacity = EXCLUDED.capacity,
    first_seen = EXCLUDED.first_seen;",
    );

    let query = query_builder.build();

    query
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to insert nodes {e:?}"))?;

    Ok(())
}

pub async fn load_nodes_endpoint(State(pool): State<PgPool>) -> Result<(), (StatusCode, String)> {
    load_nodes(&pool).await.map_err(|_| internal_error())?;

    Ok(())
}
