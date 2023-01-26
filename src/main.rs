use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
// Binds to localhost on port 8000 and runs our server on that address.
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
