use tokio::net::TcpListener;

pub async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    let pool = lightning_challenge::setup_db().await;

    tokio::spawn(async { lightning_challenge::run(listener, pool).await });

    format!("http://127.0.0.1:{}", port)
}
