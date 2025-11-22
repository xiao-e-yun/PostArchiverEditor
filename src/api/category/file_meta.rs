use post_archiver::{FileMeta, FileMetaId};
use rusqlite::Row;

use crate::api::{relation::RequireRelations, utils::ListItemResponse};

use super::Category;

impl RequireRelations for FileMeta {}

impl Category for FileMeta {
    type Id = FileMetaId;
    const TABLE_NAME: &'static str = "file_metas";
    const SEARCH_NAME: &'static str = "filename";

    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        FileMeta::from_row(row)
    }

    fn into_list_item(self) -> ListItemResponse {
        ListItemResponse {
            id: self.id.0,
            name: format!("{}/{}", self.post, self.filename),
            thumb: None,
        }
    }
}
