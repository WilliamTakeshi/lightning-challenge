use crate::helpers::spawn_app;

#[tokio::test]
async fn node_works() {
    let url = spawn_app().await;

    let client = reqwest::Client::new();

    let response0 = client
        .get(format!("{url}/nodes"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response0.status().is_success());
}
