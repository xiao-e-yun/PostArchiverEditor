use axum::Router;
use tower_http::services::ServeDir;

use crate::config::Config;

pub fn get_resource_router(config: &Config) -> Router {
    let serve_dir = ServeDir::new(&config.path);
    Router::new().fallback_service(serve_dir)
}
