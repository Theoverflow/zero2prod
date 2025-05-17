//! src/main.rs

use std::net::TcpListener;
use z2p::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    run(listener).await?.await
}
