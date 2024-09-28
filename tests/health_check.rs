use tokio::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let url = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{url}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(2), response.content_length());
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async { lightning_challenge::run(listener).await });

    format!("http://127.0.0.1:{}", port)
}
