use actix_web::HttpResponse;

pub async fn login_form() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("login.html"))
}
