use std::time::Duration;

use lightning_challenge::run;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {

    let pool = lightning_challenge::setup_db().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    run(listener, pool).await.unwrap();
}
