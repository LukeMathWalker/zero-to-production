use actix_web::{web, HttpResponse};

#[derive(serde::Serialize)]
pub struct BodyData {
    title: String,
    content: Content,
}

#[derive(serde::Serialize)]
pub struct Content {
    html: String,
    text: String,
}

pub async fn publish_newsletter(_body: web::Json<BodyData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
