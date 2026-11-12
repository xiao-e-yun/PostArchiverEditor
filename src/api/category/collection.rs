use crate::api::{
    relation::RequireRelations,
    utils::{Pagination, Totalled},
};
use optional_field::{Field, serde_optional_fields};
use post_archiver::{
    Collection, CollectionId, FileMetaId,
    manager::{PostArchiverBindExt, PostArchiverManager, UpdateCollection},
    query::{PostArchiverQueryExt, collection::CollectionSort, post::PostQuery},
};
use serde::{Deserialize, Serialize};

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
        let mut q = manager.collections().query();
        if !search.is_empty() {
            q.name.contains(search);
        }
        q.order.desc(CollectionSort::Id);
        q.pagination = pagination.to_query();
        Totalled::query(&q)
    }

    fn get_single(
        manager: &PostArchiverManager,
        id: Self::Id,
    ) -> post_archiver::error::Result<Option<Self>> {
        manager.collections().get(id)
    }

    fn delete_entity(
        manager: &PostArchiverManager,
        id: Self::Id,
    ) -> post_archiver::error::Result<()> {
        manager.bind(id).delete()
    }

    fn filter_posts(query: &mut PostQuery, id: Self::Id) {
        query.collections.insert(id);
    }
}

#[serde_optional_fields]
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateCollectionPayload {
    pub name: Option<String>,
    pub source: Field<String>,
    pub thumb: Field<FileMetaId>,
}

impl UpdateCategoryPayload<CollectionId> for UpdateCollectionPayload {
    fn apply(
        self,
        manager: &PostArchiverManager,
        id: CollectionId,
    ) -> post_archiver::error::Result<()> {
        let mut update = UpdateCollection::default();
        if let Some(name) = self.name {
            update = update.name(name);
        }
        if let Field::Present(source) = self.source {
            update = update.source(source);
        }
        if let Field::Present(thumb) = self.thumb {
            update = update.thumb(thumb);
        }
        manager.bind(id).update(update)
    }
}
