use post_archiver::{AuthorId, CollectionId, FileMetaId, PlatformId, TagId, query::Totalled};
use serde::{Deserialize, Serialize};

use super::relation::RequireRelations;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub limit: Option<u64>,
    pub page: Option<u64>,
}

impl Pagination {
    pub fn limit(&self) -> u64 {
        self.limit.unwrap_or(20)
    }

    pub fn page(&self) -> u64 {
        self.page.unwrap_or(0)
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
