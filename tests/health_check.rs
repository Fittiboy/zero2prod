use std::net::{SocketAddr, TcpListener};

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("http://{}/health_check", address.to_string()))
        .send()
        .await
        .expect("health check request should always be possible to send");

    // Assert
    assert!(response.status().is_success());
    // Response body should be empty for our health check
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> SocketAddr {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("listener should be able to bind a random port");
    let address = listener
        .local_addr()
        .expect("listener should always have an address");
    let server = zero2prod::run(listener)
        .expect("`run` should be able to bind the random address given by the OS");
    let _ = tokio::spawn(server);
    address
}
