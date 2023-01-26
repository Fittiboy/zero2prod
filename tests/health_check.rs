use reqwest::header;
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;

// Spin up a server instance, binding a random port (leveraging port `0` behavior) and return the
// `address`, including port, as a string in the form of `http://{address}`.
fn spawn_app() -> String {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("listener should be able to bind a random port");
    let address = listener
        .local_addr()
        .expect("listener should always have an address");
    let server = zero2prod::startup::run(listener)
        .expect("`run` should be able to bind the random address given by the OS");
    let _ = tokio::spawn(server);
    format!("http://{address}")
}

#[tokio::test]
// Ensure that the server is actually running, and the /health_check endpoint returns a 200 OK
// with an empty body.
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("health check request should always be possible to send");

    // Assert
    assert!(response.status().is_success());
    // Response body should be empty for our health check
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
// Ensure that sending valid form data to the /subscribe endpoint returns a 200 OK.
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app();
    let configuration = get_configuration().expect("failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("failed to connect to Postgres");
    let client = reqwest::Client::new();

    // Act
    let body = "name=Fitti&email=dev%40fitti.io";
    let response = client
        .post(&format!("{}/subscribe", app_address))
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("subscribe request should always be possible to send");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("failed to fetch saved subscription");

    assert_eq!(saved.email, "dev@fitti.io");
    assert_eq!(saved.name, "Fitti");
}

#[tokio::test]
// Ensure that various forms of missing data sent to /subscribe result in a 400 Bad Request
// response.
async fn subscribe_returns_a_400_for_missing_form_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=Fitti", "missing the email"),
        ("email=dev%40fitti.io", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscribe", address))
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("subscribe request should always be possible to send");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the form data was {}.",
            error_message
        );
    }
}
