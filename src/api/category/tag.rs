use post_archiver::{PlatformId, Tag, TagId};
use rusqlite::Row;

use crate::api::{relation::RequireRelations, utils::ListItemResponse};

use super::Category;

impl RequireRelations for Tag {
    fn platforms(&self) -> Vec<PlatformId> {
        self.platform.into_iter().collect()
    }
}

impl Category for Tag {
    type Id = TagId;
    const TABLE_NAME: &'static str = "tags";

    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Tag::from_row(row)
    }

    fn into_list_item(self) -> ListItemResponse {
        ListItemResponse {
            id: self.id.0,
            name: self.name,
            thumb: None,
        }
    }
}
