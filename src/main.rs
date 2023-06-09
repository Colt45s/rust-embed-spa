use anyhow::Result;
use axum::{
    body::{boxed, Full},
    http::{header, Method, StatusCode, Uri},
    response::{IntoResponse, Json, Response},
    routing::{get, Router},
};
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

static INDEX_HTML: &str = "index.html";

#[derive(RustEmbed)]
#[folder = "app/dist/"]
struct Assets;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/hello", get(hello_handler))
        .fallback(static_handler)
        .layer(
            CorsLayer::new()
                .allow_methods(vec![Method::GET, Method::POST])
                .allow_origin(Any),
        );
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    message: String,
}

async fn hello_handler() -> Json<Message> {
    Json(Message {
        message: "Hello, World!".to_string(),
    })
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    match Assets::get(path) {
        Some(content) => {
            let body = boxed(Full::from(content.data));
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(body)
                .expect("failed to render static file")
        }
        None => {
            if path.contains('.') {
                return not_found().await;
            }

            index_html().await
        }
    }
}

async fn index_html() -> Response {
    match Assets::get(INDEX_HTML) {
        Some(content) => {
            let body = boxed(Full::from(content.data));

            Response::builder()
                .header(header::CONTENT_TYPE, "text/html")
                .body(body)
                .expect("failed to render index.html")
        }
        None => not_found().await,
    }
}

async fn not_found() -> Response {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(boxed(Full::from("404")))
        .expect("failed to render 404")
}
