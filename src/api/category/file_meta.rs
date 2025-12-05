use std::collections::HashMap;

use post_archiver::{FileMeta, FileMetaId, PostId};
use rusqlite::{Row};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;

use crate::api::relation::RequireRelations;

use super::{Category, UpdateCategoryPayload};

impl RequireRelations for FileMeta {}

impl Category for FileMeta {
    type Id = FileMetaId;
    type UpdatePayload = UpdateFileMetaPayload;
    const TABLE_NAME: &'static str = "file_metas";
    const SEARCH_NAME: &'static str = "filename";

    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        FileMeta::from_row(row)
    }
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct UpdateFileMetaPayload {
    pub filename: Option<String>,
    pub post: Option<PostId>,
    pub mime: Option<String>,
    #[ts(type = "Record<string, any>")]
    pub extra: Option<HashMap<String, Value>>,
    #[serde(skip)]
    pub extra_json: String,
}

impl UpdateCategoryPayload for UpdateFileMetaPayload {
    const TABLE_NAME: &'static str = "file_metas";

    fn update(&mut self) -> super::UpdateContext<'_> {
        let mut ctx = super::UpdateContext::default();
        if let Some(filename) = &self.filename {
            ctx.content.push((":filename", filename));
        }
        if let Some(post) = &self.post {
            ctx.content.push((":post", post));
        }
        if let Some(mime) = &self.mime {
            ctx.content.push((":mime", mime));
        }
        if let Some(extra) = &self.extra {
            self.extra_json = serde_json::to_string(extra).unwrap();
            ctx.content.push((":extra", &self.extra_json));
        }
        ctx
    }
}
