use post_archiver::{FileMeta, FileMetaId};
use rusqlite::Row;

use crate::api::relation::RequireRelations;

use super::Category;

impl RequireRelations for FileMeta {}

impl Category for FileMeta {
    type Id = FileMetaId;
    const TABLE_NAME: &'static str = "file_metas";
    const SEARCH_NAME: &'static str = "filename";

    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        FileMeta::from_row(row)
    }
}
