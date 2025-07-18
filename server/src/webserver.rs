use axum::Router;
use axum::extract::State;
use axum::http::{Method, StatusCode, Uri, header};
use axum::response::sse::Event;
use axum::response::{Html, IntoResponse, Response, Sse};
use axum::routing::get;
use common::GameSnapShot;
use rust_embed::RustEmbed;
use std::convert::Infallible;

use tokio::sync::broadcast;
use tokio_stream::StreamExt as _;
use tokio_stream::wrappers::BroadcastStream;
use tower_http::cors::{Any, CorsLayer};

#[derive(RustEmbed)]
#[folder = "../dist/"]
struct Assets;

static INDEX_HTML: &str = "index.html";

#[derive(Clone)]
pub struct AppState {
    tx: broadcast::Sender<GameSnapShot>,
}

impl AppState {
    pub fn new(tx: broadcast::Sender<GameSnapShot>) -> AppState {
        Self { tx }
    }
}

pub async fn start_web_server(game_state: broadcast::Sender<GameSnapShot>) -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let default_port = std::env::var("PORT").unwrap_or_else(|_| "9090".to_string());

    let state = AppState::new(game_state);

    let app = Router::new()
        .route("/sse", get(sse_handler))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST]),
        )
        .fallback(static_handler)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{default_port}")).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn sse_handler(
    State(state): State<AppState>,
) -> Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>> {
    let rx = state.tx.subscribe();

    let stream = BroadcastStream::new(rx).filter_map(|msg| match msg {
        Ok(msg) => Some(Ok(
            Event::default().data(serde_json::to_string(&msg).unwrap())
        )),
        Err(_) => None,
    });

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::new())
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
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
        Some(content) => Html(content.data).into_response(),
        None => not_found().await,
    }
}

async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}
