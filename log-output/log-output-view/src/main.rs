use std::fs::File;
use std::io::prelude::*;
use std::net::SocketAddr;

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").map(|val| val.parse::<u16>().unwrap()).unwrap_or(3000);

    let app = Router::new().route("/", get(|| async move {
        let mut log_file = File::open("/shared/log.txt").expect("Failed to open file!");

        let mut log_message = String::new();
        log_file.read_to_string(&mut log_message).expect("Failure to read file!");

        let mut ping_pong_file = File::open("/shared-ping-pong/ping-pongs.txt").expect("Failed to open file!");

        let mut ping_pong_message = String::new();
        ping_pong_file.read_to_string(&mut ping_pong_message).expect("Failure to read file!");

        format!("{}\nPing / Pongs: {}", log_message, ping_pong_message)
    }));

    let addr = SocketAddr::from(([0,0,0,0], port));

    println!("Started at port {}", port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
