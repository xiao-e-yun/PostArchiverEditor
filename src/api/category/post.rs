use post_archiver::{PlatformId, Post, PostId};
use rusqlite::Row;

use crate::api::{relation::RequireRelations, utils::ListItemResponse};

use super::Category;

impl RequireRelations for Post {
    fn platforms(&self) -> Vec<PlatformId> {
        self.platform.into_iter().collect()
    }
}

impl Category for Post {
    type Id = PostId;
    const TABLE_NAME: &'static str = "posts";

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
}
