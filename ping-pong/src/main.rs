use std::{net::SocketAddr, sync::{Arc, atomic::{AtomicUsize, Ordering}}};

use std::fs::File;
use std::io::prelude::*;

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let counter: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

    update_file_contents(counter.load(Ordering::Relaxed));

    let port = std::env::var("PORT").map(|val| val.parse::<u16>().unwrap()).unwrap_or(3000);

    let counter_copy = counter.clone();
    let app = Router::new().route("/pingpong", get(|| async move {
        let value = counter_copy.fetch_add(1, Ordering::SeqCst);

        update_file_contents(value);

        format!("pong {}", value)
    }));

    let addr = SocketAddr::from(([0,0,0,0], port));

    println!("Started at port {}", port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn update_file_contents(value: usize) {
    let mut file = File::create("/shared/ping-pongs.txt").expect("Failed to open file!");

    file.write(format!("{}", value).as_bytes()).expect("Failed to write to file!");
}
