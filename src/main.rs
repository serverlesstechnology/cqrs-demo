#![forbid(unsafe_code)]
#![deny(clippy::all)]

use crate::route_handler::create_routes;

mod command_extractor;
mod config;
mod domain;
mod queries;
mod route_handler;
mod services;

#[tokio::main]
async fn main() {
    let router = create_routes().await;
    // Start the Axum server.
    axum::Server::bind(&"0.0.0.0:3030".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
