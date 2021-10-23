use actix_web::HttpResponse;

pub async fn home() -> HttpResponse {
    let body = include_str!("home.html");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}
