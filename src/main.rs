use std::net::TcpListener;
use zero2prod::startup::run;

#[tokio::main]
// Binds to localhost on port 8000 and runs our server on that address.
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    run(listener)?.await
}
