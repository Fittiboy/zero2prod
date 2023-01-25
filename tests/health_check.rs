use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    let address = listener.local_addr().unwrap();
    spawn_app(listener);
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("http://{}/health_check", address.to_string()))
        .send()
        .await
        .expect("failed to execute request");

    // Assert
    assert!(response.status().is_success());
    // Response body should be empty for our health check
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app(listener: TcpListener) {
    let server = zero2prod::run(listener).expect("failed to bind to address");
    let _ = tokio::spawn(server);
}
