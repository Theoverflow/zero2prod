//! src/main.rs

// use secrecy::ExposeSecret;
// use sqlx::PgPool;
use z2p::configurations::get_configuration;
use z2p::startup::Application;
use z2p::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("z2p".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration.");

    // let connection_pool = PgPool::connect(&config.database.connection_string().expose_secret())
    //     .await
    //     .expect("Failed to connect to Postgres.");
    // let connection_pool = PgPoolOptions::new()
    //     .connect_with(std::time::Duration::from_secs(2))
    //     .connect_lazy(&config.database.connection_string());
    // // .connect_lazy_with(&config.database.with_db());
    let application = Application::build(config).await?;
    application.run_until_stopped().await?;

    Ok(())
}
