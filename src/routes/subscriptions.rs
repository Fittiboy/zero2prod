use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SubscribeForm {
    name: String,
    email: String,
}

// Subscription POST endpoint, which should handle new subscriptoin requests, and returning a 200
// OK when valid form data, in the form of name=name&email=email is given, and a 400 Bad Requst
// otherwise.
pub async fn subscribe(form: web::Form<SubscribeForm>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
