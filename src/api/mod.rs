pub mod category;
pub mod post;
pub mod relation;
pub mod utils;

use std::sync::{Arc, Mutex};

use axum::{
    Router,
    http::StatusCode,
};
use category::Category;
use post_archiver::{Author, Collection, FileMeta, Platform, Post, Tag, manager::PostArchiverManager};
use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    manager: Arc<Mutex<PostArchiverManager>>,
}

impl AppState {
    pub fn manager(&self) -> std::sync::MutexGuard<'_, PostArchiverManager> {
        self.manager.lock().unwrap()
    }
}

pub fn get_api_router(config: &Config) -> Router<()> {
    let path = config.path.clone();

    let manager = PostArchiverManager::open(path).unwrap().unwrap();
    let manager = Arc::new(Mutex::new(manager));

    let state = AppState {
        manager,
    };

    let router = Router::new();
    let router = Post::wrap_category_route(router);
    let router = Tag::wrap_category_route(router);
    let router = Author::wrap_category_route(router);
    let router = Platform::wrap_category_route(router);
    let router = Collection::wrap_category_route(router);
    let router = FileMeta::wrap_category_route(router);

    router.fallback(StatusCode::NOT_FOUND).with_state(state)
}
