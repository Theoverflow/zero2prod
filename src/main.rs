//! src/main.rs
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use z2p::configurations::get_configuration;
use z2p::startup::run;
use z2p::telemetry;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("z2p".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration.");

    let connection_pool = PgPool::connect(&config.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool).await?.await
}
