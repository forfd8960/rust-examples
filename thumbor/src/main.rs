use serde::Deserialize;
use axum::{Router, routing::get};

mod pb;
use pb::*;

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/image/:spec/:url", get());

    let addr = "127.0.0.1:3000".parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
