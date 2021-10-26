use actix_web::http::header::LOCATION;
use actix_web::web;
use actix_web::HttpResponse;

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: String,
}

pub async fn login(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish()
}
