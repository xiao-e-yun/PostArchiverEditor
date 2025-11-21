use axum::{
    Router,
    routing::get,
};
use post_archiver::{PlatformId, Post, PostId};
use rusqlite::Row;

use crate::api::{AppState, category::list_category_handler, post::get_post_handler, relation::RequireRelations, utils::ListItemResponse};

use super::Category;

impl RequireRelations for Post {
    fn platforms(&self) -> Vec<PlatformId> {
        self.platform.into_iter().collect()
    }
}

impl Category for Post {
    type Id = PostId;

    const TABLE_NAME: &'static str = "posts";
    const SEARCH_NAME: &'static str = "title";

    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Post::from_row(row)
    }

    fn into_list_item(self) -> ListItemResponse {
        ListItemResponse {
            id: self.id.0,
            name: self.title,
            thumb: self.thumb,
        }
    }

    fn wrap_category_route(router: Router<AppState>) -> Router<AppState> {
        router
            .route(
                &format!("/{}", Self::TABLE_NAME),
                get(list_category_handler::<Self>),
            )
            .route(
                &format!("/{}/{{id}}", Self::TABLE_NAME),
                get(get_post_handler),
            )
    }
}
