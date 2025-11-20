use post_archiver::FileMetaId;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::relation::RequireRelations;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub limit: Option<u32>,
    pub page: Option<u32>,
}

impl Pagination {
    pub fn limit(&self) -> u32 {
        self.limit.unwrap_or(20)
    }

    pub fn page(&self) -> u32 {
        self.page.unwrap_or(0)
    }

    pub fn params(&self) -> [(&'static str, u32); 2] {
        let limit = self.limit();
        let page = self.page() * limit;
        [(":limit", limit), (":offset", page)]
    }
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ListResponse {
    pub list: Vec<ListItemResponse>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ListItemResponse {
    pub id: u32,
    pub name: String,
    pub thumb: Option<FileMetaId>
}

impl RequireRelations for ListResponse {
    fn file_metas(&self) -> Vec<FileMetaId> {
        self.list.iter().flat_map(|item| item.thumb.into_iter()).collect()
    }
}
