use post_archiver::{Collection, CollectionId};
use rusqlite::Row;

use crate::api::{relation::RequireRelations, utils::ListItemResponse};

use super::Category;

impl RequireRelations for Collection {
    fn file_metas(&self) -> Vec<post_archiver::FileMetaId> {
        self.thumb.into_iter().collect()
    }
}

impl Category for Collection {
    type Id = CollectionId;
    const TABLE_NAME: &'static str = "collections";

    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Collection::from_row(row)
    }

    fn into_list_item(self) -> ListItemResponse {
        ListItemResponse {
            id: self.id.0,
            name: self.name,
            thumb: self.thumb,
        }
    }
}
