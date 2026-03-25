use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::extract::Query;
use chrono::{DateTime, Utc};
use post_archiver::{
    AuthorId, CollectionId, Comment, Content, FileMetaId, PlatformId, Post, PostId, TagId,
    impl_from_query,
    query::{
        Countable, Paginate, Query as QueryTrait, SortDir, Sortable, Totalled, post::PostSort,
    },
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::api::{AppState, utils::Pagination};

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

    pub comments: Vec<Comment>,

    #[serde(skip)]
    pub tags: Vec<TagId>,
    #[serde(skip)]
    pub authors: Vec<AuthorId>,
    #[serde(skip)]
    pub collections: Vec<CollectionId>,

    #[serde(skip)]
    pub file_metas: Vec<FileMetaId>,
}

impl RequireRelations for PostResponse {
    fn tags(&self) -> Vec<TagId> {
        self.tags.clone()
    }
    fn authors(&self) -> Vec<AuthorId> {
        self.authors.clone()
    }
    fn collections(&self) -> Vec<CollectionId> {
        self.collections.clone()
    }
    fn platforms(&self) -> Vec<PlatformId> {
        self.platform.iter().cloned().collect()
    }
    fn file_metas(&self) -> Vec<FileMetaId> {
        self.file_metas.clone()
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

    let tags = manager
        .bind(id)
        .list_tags()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let authors = manager
        .bind(id)
        .list_authors()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let collections = manager
        .bind(id)
        .list_collections()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let file_metas = manager
        .bind(post.id)
        .list_file_metas()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
            file_metas,
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

impl_from_query! {
    PostShortResponse extends Post {
        id: "id",
        title: "title",
        thumb: "thumb",
        platform: "platform",
    }
}

impl RequireRelations for PostShortResponse {
    fn platforms(&self) -> Vec<PlatformId> {
        self.platform.iter().cloned().collect()
    }
    fn file_metas(&self) -> Vec<FileMetaId> {
        self.thumb.iter().cloned().collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct PostFilter {
    #[serde(default)]
    pub search: String,
    #[serde(default)]
    pub author: Option<AuthorId>,
    #[serde(default)]
    pub tag: Option<TagId>,
    #[serde(default)]
    pub collection: Option<CollectionId>,
    #[serde(default)]
    pub platform: Option<PlatformId>,
}

pub async fn list_post_handler(
    Query(filter): Query<PostFilter>,
    Query(pagination): Query<Pagination>,
    State(state): State<AppState>,
) -> Result<Json<WithRelations<Totalled<Vec<PostShortResponse>>>>, StatusCode> {
    let manager = state.manager();

    let mut query = manager.posts();

    if !filter.search.is_empty() {
        query.title.contains(&filter.search);
    }

    if let Some(author) = filter.author {
        query.authors.insert(author);
    }

    if let Some(tag) = filter.tag {
        query.tags.insert(tag);
    }

    if let Some(collection) = filter.collection {
        query.collections.insert(collection);
    }

    if let Some(platform) = filter.platform {
        query.platforms.insert(platform);
    }

    let result = query
        .sort(PostSort::Id, SortDir::Desc)
        .pagination(pagination.limit(), pagination.page())
        .with_total()
        .query::<PostShortResponse>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    WithRelations::new(&manager, result)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .map(Json::from)
}
