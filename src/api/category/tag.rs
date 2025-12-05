use post_archiver::{PlatformId, Tag, TagId};
use rusqlite::Row;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;

use crate::api::relation::RequireRelations;

use super::{Category, UpdateCategoryPayload, UpdateContext};

impl RequireRelations for Tag {
    fn platforms(&self) -> Vec<PlatformId> {
        self.platform.into_iter().collect()
    }
}

impl Category for Tag {
    type Id = TagId;
    type UpdatePayload = UpdateTagPayload;
    const TABLE_NAME: &'static str = "tags";

    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Tag::from_row(row)
    }
}

#[derive(Debug, Deserialize, Serialize, TS)]
pub struct UpdateTagPayload {
    pub name: Option<String>,
    #[ts(type = "number | null")]
    pub platform: Option<Value>,
    #[serde(skip)]
    platform_id: Option<PlatformId>,
}

impl UpdateCategoryPayload for UpdateTagPayload {
    const TABLE_NAME: &'static str = "tags";

    fn update(&mut self) -> UpdateContext<'_> {
        let mut ctx = super::UpdateContext::default();
        if let Some(name) = &self.name {
            ctx.content.push((":name", name));
        }
        if let Some(platform) = &self.platform {
            ctx.content.push((
                ":platform",
                match platform {
                    Value::Null => &rusqlite::types::Null,
                    Value::Number(num) => {
                        self.platform_id = num.as_u64().map(|n| PlatformId(n as u32));
                        match self.platform_id.as_ref() {
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
