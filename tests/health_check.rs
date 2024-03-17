use tokio::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Starting the app
    let addr = spawn_app().await;
    // Connect to endpoint
    let client = reqwest::Client::new();
    let response = client
        .get(addr)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move { zero2prod_axum::run(listener).await.unwrap() });
    format!("http://127.0.0.1:{}/health_check", port)
}
