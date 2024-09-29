use lightning_challenge::{routes::nodes::load_nodes, run};

#[tokio::main]
async fn main() {
    // TODO: Improve logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let pool = lightning_challenge::setup_db().await;
    // TODO: Remove unwraps
    // Setup database
    load_nodes(&pool).await.unwrap();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    run(listener, pool).await.unwrap();
}
