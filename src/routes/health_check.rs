use actix_web::HttpResponse;

// Basic health check endpoint to verify the server is alive.
// Always returns a 200 OK with en empty body.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
