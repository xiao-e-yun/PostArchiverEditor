use post_archiver::{Platform, PlatformId};
use rusqlite::Row;

use crate::api::{relation::RequireRelations, utils::ListItemResponse};

use super::Category;

impl RequireRelations for Platform {}

impl Category for Platform {
    type Id = PlatformId;
    const TABLE_NAME: &'static str = "platforms";

    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Platform::from_row(row)
    }

    fn into_list_item(self) -> ListItemResponse {
        ListItemResponse {
            id: self.id.0,
            name: self.name,
            thumb: None,
        }
    }
}
