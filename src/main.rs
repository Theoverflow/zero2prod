//! src/main.rs

// use secrecy::ExposeSecret;
// use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use z2p::configurations::get_configuration;
use z2p::email_client::EmailClient;
use z2p::startup::run;
use z2p::telemetry;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("z2p".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration.");

    // let connection_pool = PgPool::connect(&config.database.connection_string().expose_secret())
    //     .await
    //     .expect("Failed to connect to Postgres.");
    // let connection_pool = PgPoolOptions::new()
    //     .connect_with(std::time::Duration::from_secs(2))
    //     .connect_lazy(&config.database.connection_string());
    // // .connect_lazy_with(&config.database.with_db());

    // Create a lazy pool with the configured options
    let connection_pool = PgPoolOptions::new().connect_lazy_with(config.database.with_db());
    // Build an `EmailClient` using `configuration`
    let sender_email = config
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let email_client = EmailClient::new(
        config.email_client.base_url,
        sender_email,
        config.email_client.authorization_token,
    );
    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool, email_client).await?.await
}
