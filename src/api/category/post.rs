use axum::{Router, routing::get};
use post_archiver::{FileMetaId, PlatformId, Post, PostId};
use rusqlite::Row;

use crate::api::{
    AppState,
    post::{get_post_handler, list_post_handler},
    relation::RequireRelations,
};

use super::Category;

impl RequireRelations for Post {
    fn platforms(&self) -> Vec<PlatformId> {
        self.platform.into_iter().collect()
    }
    fn file_metas(&self) -> Vec<FileMetaId> {
        self.thumb.into_iter().collect()
    }
}

impl Category for Post {
    type Id = PostId;

    const TABLE_NAME: &'static str = "posts";
    const SEARCH_NAME: &'static str = "title";

    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Post::from_row(row)
    }

    fn wrap_category_route(router: Router<AppState>) -> Router<AppState> {
        router
            .route(&format!("/{}", Self::TABLE_NAME), get(list_post_handler))
            .route(
                &format!("/{}/{{id}}", Self::TABLE_NAME),
                get(get_post_handler),
            )
    }
}
