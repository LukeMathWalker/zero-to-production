use actix_web::HttpResponse;

pub async fn login_form() -> HttpResponse {
    HttpResponse::Ok().finish()
}
