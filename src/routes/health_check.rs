use actix_web::HttpResponse;

//TODO: Determine if this has to be async
#[allow(clippy::unused_async)]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
