use uuid::Uuid;

use chrono::prelude::*;

use std::{thread::sleep, time::Duration};

use std::net::SocketAddr;

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let string = Uuid::new_v4().hyphenated().to_string();

    let port = std::env::var("PORT").map(|val| val.parse::<u16>().unwrap()).unwrap_or(3000);

    let string_copy = string.clone();
    let app = Router::new().route("/", get(|| async move { return_with_timestamp(&string_copy) }));

    let addr = SocketAddr::from(([0,0,0,0], port));

    println!("Started at port {}", port);

    tokio::spawn(async move {
        loop {
            println!("{}", return_with_timestamp(&string));

            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


fn return_with_timestamp(value: &str) -> String {
    let time = Utc::now();

    format!("{}: {}", time, value)
}
