use anyhow::Context;
use chapter03_1::configuration::get_configuration;
use chapter03_1::startup::run;
use sqlx::postgres::PgPool;
use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> Result<(), anyhow::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .map_err(anyhow::Error::from)
        .with_context(|| "Failed to connect to Postgres.")?;
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}
