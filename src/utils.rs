use actix_web::HttpResponse;

// Return an opaque 500 while preserving the error root cause for logging.
pub fn e500<T>(e: T) -> actix_web::error::InternalError<T> {
    actix_web::error::InternalError::from_response(e, HttpResponse::InternalServerError().finish())
}
