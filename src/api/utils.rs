use post_archiver::{
    AuthorId, CollectionId, FileMetaId, PlatformId, TagId,
    query::{FromQuery, Pagination as QueryPagination, Query},
};
use serde::{Deserialize, Serialize};

use super::relation::RequireRelations;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub limit: Option<u64>,
    pub page: Option<u64>,
}

impl Pagination {
    pub fn to_query(&self) -> QueryPagination {
        QueryPagination::default()
            .limit(self.limit.unwrap_or(20))
            .page(self.page.unwrap_or(0))
    }
}

/// A page of items together with the total number of matches (ignoring pagination).
#[derive(Debug, Clone, Serialize)]
pub struct Totalled<T> {
    pub items: T,
    pub total: u64,
}

impl<T> Totalled<Vec<T>> {
    pub fn query<Q: Query>(query: &Q) -> post_archiver::error::Result<Self>
    where
        T: FromQuery<Based = Q::Source>,
    {
        Ok(Self {
            total: query.count()?,
            items: query.query()?,
        })
    }
}

impl<T: RequireRelations> RequireRelations for Totalled<Vec<T>> {
    fn authors(&self) -> Vec<AuthorId> {
        self.items.authors()
    }
    fn collections(&self) -> Vec<CollectionId> {
        self.items.collections()
    }
    fn platforms(&self) -> Vec<PlatformId> {
        self.items.platforms()
    }
    fn tags(&self) -> Vec<TagId> {
        self.items.tags()
    }
    fn file_metas(&self) -> Vec<FileMetaId> {
        self.items.file_metas()
    }
}
