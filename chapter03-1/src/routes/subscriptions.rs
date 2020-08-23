use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct SubscribeRequest {
    email: String,
    name: String,
}

pub async fn subscribe(payload: web::Json<SubscribeRequest>) -> HttpResponse {
    todo!()
}
