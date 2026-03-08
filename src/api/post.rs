use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::extract::Query;
use chrono::{DateTime, Utc};
use post_archiver::{
    Author, Collection, Comment, Content, FileMetaId, PlatformId, Post, PostId, Tag,
    query::{Countable, Paginate, Query as QueryTrait, Sortable, SortDir, Totalled, post::PostSort},
};
use serde::Serialize;
use ts_rs::TS;

use crate::api::{
    AppState,
    category::Filter,
    utils::Pagination,
};

use super::relation::{RequireRelations, WithRelations};

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct PostResponse {
    pub id: PostId,
    pub title: String,
    pub content: Vec<Content>,
    pub source: Option<String>,
    pub updated: DateTime<Utc>,
    pub published: DateTime<Utc>,
    pub thumb: Option<FileMetaId>,
    pub platform: Option<PlatformId>,

    pub tags: Vec<Tag>,
    pub authors: Vec<Author>,
    pub collections: Vec<Collection>,
    pub comments: Vec<Comment>,
}

impl RequireRelations for PostResponse {
    fn platforms(&self) -> Vec<PlatformId> {
        self.platform
            .iter()
            .cloned()
            .chain(self.tags.iter().filter_map(|a| a.platform))
            .collect()
    }
    fn file_metas(&self) -> Vec<FileMetaId> {
        self.content
            .iter()
            .filter_map(|content| match content {
                Content::File(file_meta) => Some(*file_meta),
                _ => None,
            })
            .chain(self.thumb.iter().cloned())
            .chain(self.authors.iter().flat_map(|a| a.thumb))
            .chain(self.collections.iter().flat_map(|c| c.thumb))
            .collect()
    }
}

pub async fn get_post_handler(
    Path(id): Path<PostId>,
    State(state): State<AppState>,
) -> Result<Json<WithRelations<PostResponse>>, StatusCode> {
    let manager = state.manager();

    let post = manager
        .get_post(id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let tag_ids = manager.bind(id).list_tags().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let author_ids = manager.bind(id).list_authors().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let collection_ids = manager.bind(id).list_collections().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let tags = if tag_ids.is_empty() {
        Vec::new()
    } else {
        let mut q = manager.tags();
        q.ids.extend(tag_ids);
        q.query::<Tag>().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    let authors = if author_ids.is_empty() {
        Vec::new()
    } else {
        let mut q = manager.authors();
        q.ids.extend(author_ids);
        q.query::<Author>().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    let collections = if collection_ids.is_empty() {
        Vec::new()
    } else {
        let mut q = manager.collections();
        q.ids.extend(collection_ids);
        q.query::<Collection>().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    WithRelations::new(
        &manager,
        PostResponse {
            id: post.id,
            title: post.title,
            content: post.content,
            thumb: post.thumb,
            platform: post.platform,
            source: post.source,
            updated: post.updated,
            published: post.published,
            comments: post.comments,
            tags,
            authors,
            collections,
        },
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    .map(Json::from)
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct PostShortResponse {
    pub id: PostId,
    pub title: String,
    pub thumb: Option<FileMetaId>,
    pub platform: Option<PlatformId>,
}

impl RequireRelations for PostShortResponse {
    fn platforms(&self) -> Vec<PlatformId> {
        self.platform.iter().cloned().collect()
    }
    fn file_metas(&self) -> Vec<FileMetaId> {
        self.thumb.iter().cloned().collect()
    }
}

pub async fn list_post_handler(
    Query(filter): Query<Filter>,
    Query(pagination): Query<Pagination>,
    State(state): State<AppState>,
) -> Result<Json<WithRelations<Totalled<Vec<PostShortResponse>>>>, StatusCode> {
    let manager = state.manager();
    let search = &filter.search;

    let mut query = manager.posts();
    if !search.is_empty() {
        query.title.contains(search);
    }

    let result = query
        .sort(PostSort::Published, SortDir::Desc)
        .pagination(pagination.limit() as u64, pagination.page() as u64)
        .with_total()
        .query::<Post>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let totalled = Totalled {
        items: result.items.into_iter().map(|post| PostShortResponse {
            id: post.id,
            title: post.title,
            thumb: post.thumb,
            platform: post.platform,
        }).collect(),
        total: result.total,
    };

    WithRelations::new(&manager, totalled)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .map(Json::from)
}
