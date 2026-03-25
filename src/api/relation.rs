use std::{collections::HashSet, fmt::Debug};

use post_archiver::{
    Author, AuthorId, Collection, CollectionId, FileMeta, FileMetaId, Platform, PlatformId, Tag,
    TagId, manager::PostArchiverManager, query::Query,
};
use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct WithRelations<T: Debug> {
    #[serde(flatten)]
    pub inner: T,
    pub authors: Vec<Author>,
    pub collections: Vec<Collection>,
    pub platforms: Vec<Platform>,
    pub tags: Vec<Tag>,
    pub file_metas: Vec<FileMeta>,
}

impl<T: Debug + RequireRelations> WithRelations<T> {
    pub fn new(manager: &PostArchiverManager, inner: T) -> post_archiver::error::Result<Self> {
        let author_ids = inner.authors();
        let authors = if author_ids.is_empty() {
            vec![]
        } else {
            let mut q = manager.authors();
            q.ids.extend(author_ids);
            q.query::<Author>()?
        };

        let collection_ids = inner.collections();
        let collections = if collection_ids.is_empty() {
            vec![]
        } else {
            let mut q = manager.collections();
            q.ids.extend(collection_ids);
            q.query::<Collection>()?
        };

        let tag_ids = inner.tags();
        let tags = if tag_ids.is_empty() {
            vec![]
        } else {
            let mut q = manager.tags();
            q.ids.extend(tag_ids);
            q.query::<Tag>()?
        };

        let platform_ids: HashSet<PlatformId> = inner
            .platforms()
            .into_iter()
            .chain(tags.iter().flat_map(|t| t.platform))
            .collect();
        let platforms = if platform_ids.is_empty() {
            vec![]
        } else {
            let mut q = manager.platforms();
            q.ids.extend(platform_ids);
            q.query::<Platform>()?
        };

        let file_meta_ids: HashSet<FileMetaId> = inner
            .file_metas()
            .into_iter()
            .chain(authors.iter().flat_map(|a| a.thumb))
            .chain(collections.iter().flat_map(|c| c.thumb))
            .collect();
        let file_metas: Vec<FileMeta> = file_meta_ids
            .into_iter()
            .filter_map(|id| manager.get_file_meta(id).ok().flatten())
            .collect();

        Ok(Self {
            inner,
            authors,
            collections,
            platforms,
            tags,
            file_metas,
        })
    }
}

pub trait RequireRelations {
    fn authors(&self) -> Vec<AuthorId> {
        vec![]
    }
    fn collections(&self) -> Vec<CollectionId> {
        vec![]
    }
    fn platforms(&self) -> Vec<PlatformId> {
        vec![]
    }
    fn tags(&self) -> Vec<TagId> {
        vec![]
    }
    fn file_metas(&self) -> Vec<FileMetaId> {
        vec![]
    }
}

impl<T: RequireRelations> RequireRelations for Vec<T> {
    fn authors(&self) -> Vec<AuthorId> {
        self.iter().flat_map(|item| item.authors()).collect()
    }
    fn collections(&self) -> Vec<CollectionId> {
        self.iter().flat_map(|item| item.collections()).collect()
    }
    fn platforms(&self) -> Vec<PlatformId> {
        self.iter().flat_map(|item| item.platforms()).collect()
    }
    fn tags(&self) -> Vec<TagId> {
        self.iter().flat_map(|item| item.tags()).collect()
    }
    fn file_metas(&self) -> Vec<FileMetaId> {
        self.iter().flat_map(|item| item.file_metas()).collect()
    }
}
