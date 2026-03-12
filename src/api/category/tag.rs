use post_archiver::{
    PlatformId, Tag, TagId,
    manager::{PostArchiverManager, UpdateTag},
    query::{Totalled, Paginate, Countable, Query},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;

use crate::api::{
    relation::RequireRelations,
    utils::Pagination,
};

use super::{Category, UpdateCategoryPayload};

impl RequireRelations for Tag {
    fn platforms(&self) -> Vec<PlatformId> {
        self.platform.into_iter().collect()
    }
}

impl Category for Tag {
    type Id = TagId;
    type UpdatePayload = UpdateTagPayload;

    const ROUTE: &'static str = "tags";

    fn list_query(
        manager: &PostArchiverManager,
        pagination: &Pagination,
        search: &str,
    ) -> post_archiver::error::Result<Totalled<Vec<Self>>> {
        let mut q = manager.tags();
        if !search.is_empty() {
            q.name.contains(search);
        }
        q.pagination(pagination.limit(), pagination.page())
            .with_total()
            .query::<Tag>()
    }

    fn get_single(
        manager: &PostArchiverManager,
        id: Self::Id,
    ) -> post_archiver::error::Result<Option<Self>> {
        manager.get_tag(id)
    }

    fn delete_entity(
        manager: &PostArchiverManager,
        id: Self::Id,
    ) -> post_archiver::error::Result<()> {
        manager.bind(id).delete()
    }

    fn filter_posts<T>(mut query: post_archiver::query::post::PostQuery<T>, id: Self::Id) -> post_archiver::query::post::PostQuery<T> {
        query.tags.insert(id);
        query
    }
}

#[derive(Debug, Deserialize, Serialize, TS)]
pub struct UpdateTagPayload {
    pub name: Option<String>,
    #[ts(type = "number | null")]
    pub platform: Option<Value>,
}

impl UpdateCategoryPayload<TagId> for UpdateTagPayload {
    fn apply(self, manager: &PostArchiverManager, id: TagId) -> post_archiver::error::Result<()> {
        let mut update = UpdateTag::default();
        if let Some(name) = self.name {
            update = update.name(name);
        }
        if let Some(platform) = self.platform {
            update = update.platform(match platform {
                Value::Null => None,
                Value::Number(n) => n.as_u64().map(|n| PlatformId(n as u32)),
                _ => None,
            });
        }
        manager.bind(id).update(update)
    }
}
