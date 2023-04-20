use axum::routing::get;
use axum::Router;
use cqrs_demo::route_handler::{command_handler, query_handler};
use cqrs_demo::state::new_application_state;

#[tokio::main]
async fn main() {
    let recorder: metrics_exporter_statsd::StatsdRecorder =
        metrics_exporter_statsd::StatsdBuilder::from("127.0.0.1", 8125)
            .with_queue_size(5000)
            .with_buffer_size(1024)
            .build(Some("test-"))
            .expect("Could not create StatsdRecorder");
    metrics::set_boxed_recorder(Box::new(recorder)).unwrap();

    let state = new_application_state().await;
    // Configure the Axum routes and services.
    // For this example a single logical endpoint is used and the HTTP method
    // distinguishes whether the call is a command or a query.
    let router = Router::new()
        .route(
            "/account/:account_id",
            get(query_handler).post(command_handler),
        )
        .with_state(state);
    // Start the Axum server.
    axum::Server::bind(&"0.0.0.0:3030".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
