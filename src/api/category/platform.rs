use post_archiver::{
    Platform, PlatformId,
    manager::{PostArchiverManager, UpdatePlatform},
    query::{Totalled, Paginate, Countable, Query},
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::api::{
    relation::RequireRelations,
    utils::Pagination,
};

use super::{Category, UpdateCategoryPayload};

impl RequireRelations for Platform {}

impl Category for Platform {
    type Id = PlatformId;
    type UpdatePayload = UpdatePlatformPayload;

    const ROUTE: &'static str = "platforms";

    fn list_query(
        manager: &PostArchiverManager,
        pagination: &Pagination,
        search: &str,
    ) -> post_archiver::error::Result<Totalled<Vec<Self>>> {
        let mut q = manager.platforms();
        if !search.is_empty() {
            q.name.contains(search);
        }
        q.pagination(pagination.limit(), pagination.page())
            .with_total()
            .query::<Platform>()
    }

    fn get_single(
        manager: &PostArchiverManager,
        id: Self::Id,
    ) -> post_archiver::error::Result<Option<Self>> {
        manager.get_platform(id)
    }

    fn delete_entity(
        manager: &PostArchiverManager,
        id: Self::Id,
    ) -> post_archiver::error::Result<()> {
        manager.bind(id).delete()
    }

    fn filter_posts<T>(mut query: post_archiver::query::post::PostQuery<T>, id: Self::Id) -> post_archiver::query::post::PostQuery<T> {
        query.platforms.insert(id);
        query
    }
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct UpdatePlatformPayload {
    pub name: Option<String>,
}

impl UpdateCategoryPayload<PlatformId> for UpdatePlatformPayload {
    fn apply(self, manager: &PostArchiverManager, id: PlatformId) -> post_archiver::error::Result<()> {
        let update = if let Some(name) = self.name {
            UpdatePlatform::default().name(name)
        } else {
            UpdatePlatform::default()
        };
        manager.bind(id).update(update)
    }
}
