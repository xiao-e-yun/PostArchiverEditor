use post_archiver::{Collection, CollectionId, FileMetaId};
use rusqlite::Row;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;

use crate::api::relation::RequireRelations;

use super::{Category, UpdateCategoryPayload};

impl RequireRelations for Collection {
    fn file_metas(&self) -> Vec<post_archiver::FileMetaId> {
        self.thumb.into_iter().collect()
    }
}

impl Category for Collection {
    type Id = CollectionId;
    type UpdatePayload = UpdateCollectionPayload;
    const TABLE_NAME: &'static str = "collections";

    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Collection::from_row(row)
    }
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct UpdateCollectionPayload {
    pub id: Option<CollectionId>,
    pub name: Option<String>,
    #[ts(type = "string | null")]
    pub source: Option<Value>,
    #[serde(skip)]
    pub source_str: String,
    #[ts(type = "number | null")]
    pub thumb: Option<Value>,
    #[serde(skip)]
    pub thumb_id: Option<FileMetaId>,
}

impl UpdateCategoryPayload for UpdateCollectionPayload {
    const TABLE_NAME: &'static str = "collections";

    fn update(&mut self) -> super::UpdateContext<'_> {
        let mut ctx = super::UpdateContext::default();
        if let Some(id) = &self.id {
            ctx.content.push((":id", id));
        }
        if let Some(name) = &self.name {
            ctx.content.push((":name", name));
        }
        if let Some(source) = &self.source {
            self.source_str = match source {
                Value::Null => "".to_string(),
                Value::String(s) => s.clone(),
                _ => "".to_string(),
            };
            ctx.content.push((":source", &self.source_str));
        }
        if let Some(thumb) = &self.thumb {
            ctx.content.push((
                ":thumb",
                match thumb {
                    Value::Null => &rusqlite::types::Null,
                    Value::Number(num) => {
                        self.thumb_id = num.as_u64().map(|n| FileMetaId(n as u32));
                        match self.thumb_id.as_ref() {
                            Some(id) => id,
                            None => &rusqlite::types::Null,
                        }
                    }
                    _ => &rusqlite::types::Null,
                },
            ));
        }
        ctx
    }
}
