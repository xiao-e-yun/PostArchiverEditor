use post_archiver::{
    Collection, CollectionId, FileMetaId,
    manager::{PostArchiverManager, UpdateCollection},
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

impl RequireRelations for Collection {
    fn file_metas(&self) -> Vec<FileMetaId> {
        self.thumb.into_iter().collect()
    }
}

impl Category for Collection {
    type Id = CollectionId;
    type UpdatePayload = UpdateCollectionPayload;

    const ROUTE: &'static str = "collections";

    fn list_query(
        manager: &PostArchiverManager,
        pagination: &Pagination,
        search: &str,
    ) -> post_archiver::error::Result<Totalled<Vec<Self>>> {
        let mut q = manager.collections();
        if !search.is_empty() {
            q.name.contains(search);
        }
        q.pagination(pagination.limit(), pagination.page())
            .with_total()
            .query::<Collection>()
    }

    fn get_single(
        manager: &PostArchiverManager,
        id: Self::Id,
    ) -> post_archiver::error::Result<Option<Self>> {
        manager.get_collection(id)
    }

    fn delete_entity(
        manager: &PostArchiverManager,
        id: Self::Id,
    ) -> post_archiver::error::Result<()> {
        manager.bind(id).delete()
    }

    fn filter_posts<T>(mut query: post_archiver::query::post::PostQuery<T>, id: Self::Id) -> post_archiver::query::post::PostQuery<T> {
        query.collections.insert(id);
        query
    }
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct UpdateCollectionPayload {
    pub name: Option<String>,
    #[ts(type = "string | null")]
    pub source: Option<Value>,
    #[ts(type = "number | null")]
    pub thumb: Option<Value>,
}

impl UpdateCategoryPayload<CollectionId> for UpdateCollectionPayload {
    fn apply(self, manager: &PostArchiverManager, id: CollectionId) -> post_archiver::error::Result<()> {
        let mut update = UpdateCollection::default();
        if let Some(name) = self.name {
            update = update.name(name);
        }
        if let Some(source) = self.source {
            update = update.source(match source {
                Value::Null => None,
                Value::String(s) => Some(s),
                _ => None,
            });
        }
        if let Some(thumb) = self.thumb {
            update = update.thumb(match thumb {
                Value::Null => None,
                Value::Number(n) => n.as_u64().map(|n| FileMetaId(n as u32)),
                _ => None,
            });
        }
        manager.bind(id).update(update)
    }
}
