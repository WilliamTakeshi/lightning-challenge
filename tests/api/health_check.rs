use crate::helpers::spawn_app;

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
