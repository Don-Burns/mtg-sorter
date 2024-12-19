use axum::{Router, routing::get};
use tokio;

use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[tokio::main]
async fn main() {
    // logging configuration
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // web server configuration
    let app = Router::new()
        // Serve the frontend files so they can be accessed as "/..." in the JS/HTML
        .nest_service("/", ServeDir::new("frontend/dist"))
        // Backend API
        .route("/server", get(hello_world))
        // logging/tracing, use trace::info!("...") to log
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind port");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

async fn hello_world() -> &'static str {
    "Hello World!"
}
