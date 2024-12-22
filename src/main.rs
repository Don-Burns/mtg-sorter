use axum::routing::post;
use axum::{Router, routing::get};
use tokio;
use tokio::io::AsyncWriteExt;

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
        .route("/upload", post(upload_file))
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

/// Just a simple hello world function for testing routes or other uses
async fn hello_world() -> &'static str {
    "Hello World!"
}

/// Upload a file to the server
async fn upload_file(mut file: axum::extract::Multipart) -> axum::response::Redirect {
    let mut server_file =
        tokio::fs::File::create("/home/donal/src/delver_sorter/file_upload_testing/file")
            .await
            .expect("Failed to create file on server to upload to");
    while let Some(field) = file
        .next_field()
        .await
        .expect("Failed getting next chunk of upload")
    {
        let name = field.name().expect("Failed getting field name").to_string();
        let data = field
            .bytes()
            .await
            .expect("Failed getting bytes from field");
        tracing::info!(
            "File uploaded! name={}, data size (bytes)={}",
            name,
            data.len()
        );

        if let Err(error) = server_file.write(data.as_ref()).await {
            tracing::error!("Failed to write to file: {}", error);
            // if the upload fails for any reason, redirect to an error page
            return axum::response::Redirect::to("/upload_error");
        };
    }

    // Redirect to a success page once upload is done
    axum::response::Redirect::to("/upload_success.html")
}
