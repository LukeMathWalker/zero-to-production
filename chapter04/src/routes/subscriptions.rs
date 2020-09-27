use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing_futures::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscribeRequest {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(payload, pool),
    fields(
        request_id=%Uuid::new_v4(),
        email = %payload.email,
        name = %payload.name
    )
)]
pub async fn subscribe(
    payload: web::Form<SubscribeRequest>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let query_span = tracing::info_span!("Saving new subscriber details in the database");
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
            "#,
        Uuid::new_v4(),
        payload.email,
        payload.name,
        Utc::now()
    )
    .execute(pool.as_ref())
    .instrument(query_span)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().finish())
}
