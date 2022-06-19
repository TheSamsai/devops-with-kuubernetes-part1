use std::{net::SocketAddr, time::Instant, sync::Arc};

use axum::{
    routing::{get, get_service},
    Router, response::{IntoResponse, Html}, http::StatusCode, Extension,
};

use tower_http::services::ServeFile;

use tokio::{process::Command, sync::Mutex};

use tera::Tera;
use tera::Context;

use lazy_static::lazy_static;

type ImageAge = Arc<Mutex<Instant>>;
type ImageStorage = Arc<String>;

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").map(|val| val.parse::<u16>().unwrap()).unwrap_or(3000);
    let image_storage = Arc::new(std::env::var("IMAGE_DIR").unwrap_or(String::from("./image")));

    let image_age: ImageAge = Arc::new(Mutex::new(Instant::now()));
    download_image_of_the_day(image_storage.clone()).await;

    let image_storage_path: String = image_storage.to_string();

    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/image", get_service(ServeFile::new(format!("{}/image.jpg", image_storage_path))).handle_error(handle_error))
        .route("/", get(index_page))
        .layer(Extension(image_storage))
        .layer(Extension(image_age))
        .layer(Extension(tera));

    let addr = SocketAddr::from(([0,0,0,0], port));

    println!("Started at port {}", port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index_page(
    Extension(image_age_state): Extension<ImageAge>,
    Extension(image_storage): Extension<ImageStorage>,
    Extension(tera): Extension<Tera>
) -> Html<String> {
    let mut image_age = image_age_state.lock().await;

    if Instant::now().duration_since(*image_age).as_secs() > 24 * 60 * 60 {
        download_image_of_the_day(image_storage).await;
        *image_age = Instant::now();
    }

    let mut context = Context::new();

    Html(tera.render("index.html", &context).unwrap())
}

async fn download_image_of_the_day(image_dir: Arc<String>) {
    Command::new("wget")
        .arg("https://picsum.photos/1200")
        .arg("-O")
        .arg(format!("{}/image.jpg", image_dir))
        .spawn()
        .expect("Failed to start 'wget'")
        .wait()
        .await
        .expect("'wget' failed to run");
}

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
