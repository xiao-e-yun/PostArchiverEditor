use crate::api::{relation::RequireRelations, utils::Pagination};
use optional_field::{Field, serde_optional_fields};
use post_archiver::{
    Collection, CollectionId, FileMetaId,
    manager::{PostArchiverManager, UpdateCollection},
    query::{Countable, Paginate, Query, SortDir, Sortable, Totalled, collection::CollectionSort},
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
        let mut q = manager.collections();
        if !search.is_empty() {
            q.name.contains(search);
        }
        q.sort(CollectionSort::Id, SortDir::Desc)
            .pagination(pagination.limit(), pagination.page())
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

    fn filter_posts<T>(
        mut query: post_archiver::query::post::PostQuery<T>,
        id: Self::Id,
    ) -> post_archiver::query::post::PostQuery<T> {
        query.collections.insert(id);
        query
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
