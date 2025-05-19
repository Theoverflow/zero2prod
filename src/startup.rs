//! src//startup.rs

use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub async fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
    // let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
    //     .bind("127.0.0.1:8083")?
    //     .run();
    let connection_pool = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection_pool.clone())
    })
    .listen(listener)?
    .run();
    // No .await here!
    Ok(server)
}
