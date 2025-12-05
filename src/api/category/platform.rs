use post_archiver::{Platform, PlatformId};
use rusqlite::Row;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::api::relation::RequireRelations;

use super::{Category, UpdateCategoryPayload, UpdateContext};

impl RequireRelations for Platform {}

impl Category for Platform {
    type Id = PlatformId;
    type UpdatePayload = UpdatePlatformPayload;
    const TABLE_NAME: &'static str = "platforms";

    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Platform::from_row(row)
    }
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct UpdatePlatformPayload {
    pub name: Option<String>,
}

impl UpdateCategoryPayload for UpdatePlatformPayload {
    const TABLE_NAME: &'static str = "platforms";

    fn update(&mut self) -> UpdateContext<'_> {
        let mut ctx = UpdateContext::default();
        if let Some(name) = &self.name {
            ctx.content.push((":name", name));
        }
        ctx
    }
}
