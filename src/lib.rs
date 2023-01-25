use actix_web::dev::Server;
use actix_web::http::StatusCode;
use actix_web::{web, App, HttpResponse, HttpResponseBuilder, HttpServer};
use serde::Deserialize;
use std::net::TcpListener;

// Basic health check endpoint to verify the server is alive.
// Always returns a 200 OK with en empty body.
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize, Debug)]
struct SubscribeForm {
    name: String,
    email: String,
}

// Subscription POST endpoint, which should handle new subscriptoin requests, and returning a 200
// OK when valid form data, in the form of name=name&email=email is given, and a 400 Bad Requst
// otherwise.
async fn subscribe(form: web::Form<SubscribeForm>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

// Takes a `TcpListener` as input and starts our server on the address the `listener` is bound to.
// Then returns the server in a Result<Server>.
pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
