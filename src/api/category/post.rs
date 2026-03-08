use axum::{Router, routing::get};
use chrono::{DateTime, Utc};
use post_archiver::{
    AuthorId, CollectionId, Comment, Content, FileMetaId, PlatformId, Post, PostId, TagId,
    manager::{PostArchiverManager, UpdatePost},
    query::{Totalled, Paginate, Countable, Query, Sortable, SortDir, post::PostSort},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;

use crate::api::{
    AppState,
    category::{delete_category_handler, update_category_handler},
    post::{get_post_handler, list_post_handler},
    relation::RequireRelations,
    utils::Pagination,
};

use super::{Category, UpdateCategoryPayload};

impl RequireRelations for Post {
    fn platforms(&self) -> Vec<PlatformId> {
        self.platform.into_iter().collect()
    }
    fn file_metas(&self) -> Vec<FileMetaId> {
        self.thumb.into_iter().collect()
    }
}

impl Category for Post {
    type Id = PostId;
    type UpdatePayload = UpdatePostPayload;

    const ROUTE: &'static str = "posts";

    fn list_query(
        manager: &PostArchiverManager,
        pagination: &Pagination,
        search: &str,
    ) -> post_archiver::error::Result<Totalled<Vec<Self>>> {
        let mut q = manager.posts();
        if !search.is_empty() {
            q.title.contains(search);
        }
        q.sort(PostSort::Updated, SortDir::Desc)
            .pagination(pagination.limit() as u64, pagination.page() as u64)
            .with_total()
            .query::<Post>()
    }

    fn get_single(
        manager: &PostArchiverManager,
        id: Self::Id,
    ) -> post_archiver::error::Result<Option<Self>> {
        manager.get_post(id)
    }

    fn delete_entity(
        manager: &PostArchiverManager,
        id: Self::Id,
    ) -> post_archiver::error::Result<()> {
        manager.bind(id).delete()
    }

    fn wrap_category_route(router: Router<AppState>) -> Router<AppState> {
        router
            .route(&format!("/{}", Self::ROUTE), get(list_post_handler))
            .route(
                &format!("/{}/{{id}}", Self::ROUTE),
                get(get_post_handler)
                    .delete(delete_category_handler::<Self>)
                    .patch(update_category_handler::<Self>),
            )
    }
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct UpdatePostPayload {
    pub title: Option<String>,
    #[ts(type = "string | null")]
    pub source: Option<Value>,
    pub content: Option<Vec<Content>>,
    #[ts(type = "number | null")]
    pub thumb: Option<Value>,
    pub comments: Option<Vec<Comment>>,
    pub updated: Option<DateTime<Utc>>,
    pub published: Option<DateTime<Utc>>,
    #[ts(type = "number | null")]
    pub platform: Option<Value>,
    // Relations
    pub authors: Option<Vec<u32>>,
    pub collections: Option<Vec<u32>>,
    pub tags: Option<Vec<u32>>,
}

impl UpdateCategoryPayload<PostId> for UpdatePostPayload {
    fn apply(self, manager: &PostArchiverManager, id: PostId) -> post_archiver::error::Result<()> {
        let mut update = UpdatePost::default();
        if let Some(title) = self.title {
            update = update.title(title);
        }
        if let Some(source) = self.source {
            update = update.source(match source {
                Value::Null => None,
                Value::String(s) => Some(s),
                _ => None,
            });
        }
        if let Some(content) = self.content {
            update = update.content(content);
        }
        if let Some(thumb) = self.thumb {
            update = update.thumb(match thumb {
                Value::Null => None,
                Value::Number(n) => n.as_u64().map(|n| FileMetaId(n as u32)),
                _ => None,
            });
        }
        if let Some(comments) = self.comments {
            update = update.comments(comments);
        }
        if let Some(updated) = self.updated {
            update = update.updated(updated);
        }
        if let Some(published) = self.published {
            update = update.published(published);
        }
        if let Some(platform) = self.platform {
            update = update.platform(match platform {
                Value::Null => None,
                Value::Number(n) => n.as_u64().map(|n| PlatformId(n as u32)),
                _ => None,
            });
        }
        manager.bind(id).update(update)?;

        let bound = manager.bind(id);
        if let Some(authors) = self.authors {
            let new_ids: Vec<AuthorId> = authors.into_iter().map(AuthorId::from).collect();
            let current = bound.list_authors()?;
            let to_remove: Vec<_> = current.iter().copied().filter(|a| !new_ids.contains(a)).collect();
            let to_add: Vec<_> = new_ids.iter().copied().filter(|a| !current.contains(a)).collect();
            bound.remove_authors(&to_remove)?;
            bound.add_authors(&to_add)?;
        }
        if let Some(tags) = self.tags {
            let new_ids: Vec<TagId> = tags.into_iter().map(TagId::from).collect();
            let current = bound.list_tags()?;
            let to_remove: Vec<_> = current.iter().copied().filter(|t| !new_ids.contains(t)).collect();
            let to_add: Vec<_> = new_ids.iter().copied().filter(|t| !current.contains(t)).collect();
            bound.remove_tags(&to_remove)?;
            bound.add_tags(&to_add)?;
        }
        if let Some(collections) = self.collections {
            let new_ids: Vec<CollectionId> = collections.into_iter().map(CollectionId::from).collect();
            let current = bound.list_collections()?;
            let to_remove: Vec<_> = current.iter().copied().filter(|c| !new_ids.contains(c)).collect();
            let to_add: Vec<_> = new_ids.iter().copied().filter(|c| !current.contains(c)).collect();
            bound.remove_collections(&to_remove)?;
            bound.add_collections(&to_add)?;
        }

        Ok(())
    }
}
