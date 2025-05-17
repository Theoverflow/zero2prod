use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use std::net::TcpListener;

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
    //     .bind("127.0.0.1:8083")?
    //     .run();
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    // No .await here!
    Ok(server)
}
