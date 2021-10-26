use actix_web::HttpResponse;

pub async fn home() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("home.html"))
}
