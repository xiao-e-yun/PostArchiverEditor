use axum::{http::Uri, response::{Html, IntoResponse}, routing::get, Router};
use rust_embed::Embed;
use tracing::info;

#[derive(Embed)]
#[folder = "frontend/dist/"]
struct Assets;

pub fn frontend() -> Router<()> {
    if cfg!(debug_assertions) {
        use axum_reverse_proxy::ReverseProxy;
        let proxy: ReverseProxy = ReverseProxy::new("/", "http://localhost:5173");
        info!("Running in debug mode");
        info!("Proxying to localhost:5173");
        Router::from(proxy)
    } else {
        Router::new().fallback(get(static_handler))
    }
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if !path.is_empty() && path != INDEX_HTML && let Some(content) = Assets::get(path)
    {
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        let headers = [(axum::http::header::CONTENT_TYPE, mime.as_ref())];
        (headers, content.data).into_response()
    } else {
        let index_html = Assets::get(INDEX_HTML).unwrap().data.into_owned();
        Html(index_html).into_response()
    }
}

const INDEX_HTML: &str = "index.html";
