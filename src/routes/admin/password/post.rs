use crate::session_state::TypedSession;
use crate::utils::e500;
use actix_web::http::header::LOCATION;
use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    old_password: String,
    new_password: String,
    new_password_check: String,
}

pub async fn change_password(
    form: web::Form<FormData>,
    session: TypedSession,
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .finish());
    };
    todo!()
}
