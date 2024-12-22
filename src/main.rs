use axum::http::uri::PathAndQuery;
use axum::http::{Error, Request, StatusCode, Uri};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Router, routing::get};
use tokio;
use tokio::io::AsyncWriteExt;

use std::net::SocketAddr;
use std::str::FromStr;
use tower::ServiceExt;
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
        .nest_service("/", get(static_file_uri_handler))
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
    axum::response::Redirect::to("/upload_success")
}

/// When using Tower's serveDir, it will serve the files as if they were in the root directory
/// This includes the file extension, meaning that if I want to redirect to another page
/// on the frontend I need to include the file extension in the redirect URL
/// and that the file extension will be visible in the URL for users
/// This is not ideal so in the case of a url not found instead of returning a 404
/// first try to find the file minus the file extension
/// This solution greatly inspired by https://github.com/tokio-rs/axum/discussions/446
async fn static_file_uri_handler(uri: Uri) -> Result<impl IntoResponse, (StatusCode, String)> {
    match get_static_file(uri.clone()).await {
        Err(_) => {}
        Ok(r) => {
            if r.into_response().status() != StatusCode::NOT_FOUND {
                // TODO: Need less crappy solution to this, just fighting the borrow checker
                return Ok(get_static_file(uri).await.unwrap());
            }
        }
    }
    let mut uri_parts = uri.clone().into_parts();
    let path_and_query = uri_parts.path_and_query;
    let (uri_path, uri_params) = match &path_and_query {
        Some(path_and_query) => (path_and_query.path(), path_and_query.query()),
        None => {
            tracing::error!("Failed to get path and query from uri: {:?}", uri);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get path and query".to_string(),
            ));
        }
    };

    uri_parts.path_and_query = Some(
        PathAndQuery::from_str(
            // only add params if present
            format!("{}{}", format!("{}.html", uri_path), match uri_params {
                Some(p) => format!("?{}", p),
                None => "".to_string(),
            })
            .as_str(),
        )
        .unwrap(),
    );
    let new_uri = Uri::from_parts(uri_parts).unwrap();

    tracing::info!(
        "uri identified as potential static file, converted uri: {}",
        new_uri
    );

    // check if the uri path corresponds to a static file
    // TODO: should consider caching here so checkups don't need to be done on all requests
    match get_static_file(new_uri).await {
        Ok(file) => Ok(file),
        Err(_) => {
            tracing::error!("Failed to find static file for uri: {}", uri);
            Err((StatusCode::NOT_FOUND, "Page not found".to_string()))
        }
    }
}

async fn get_static_file(uri: Uri) -> Result<impl IntoResponse, Error> {
    let req = Request::builder().uri(uri).body(())?;
    Ok(ServeDir::new("frontend/dist").oneshot(req).await)
}
