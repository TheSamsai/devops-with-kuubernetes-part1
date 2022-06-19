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
        let mut file = File::open("/shared/log.txt").expect("Failed to open file!");

        let mut message = String::new();
        file.read_to_string(&mut message).expect("Failure to read file!");

        message
    }));

    let addr = SocketAddr::from(([0,0,0,0], port));

    println!("Started at port {}", port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
