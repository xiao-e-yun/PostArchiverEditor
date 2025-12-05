use axum::{
    Router,
    routing::{delete, get, patch},
};
use chrono::{DateTime, Utc};
use post_archiver::{
    Comment, Content, FileMetaId, PlatformId, Post, PostId,
};
use rusqlite::Row;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;

use crate::api::{
    AppState,
    category::{delete_category_handler, update_category_handler},
    post::{get_post_handler, list_post_handler},
    relation::RequireRelations,
};

use super::{Category, RelationPayload, UpdateCategoryPayload, UpdateContext};

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

    const TABLE_NAME: &'static str = "posts";
    const SEARCH_NAME: &'static str = "title";

    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Post::from_row(row)
    }

    fn wrap_category_route(router: Router<AppState>) -> Router<AppState> {
        router
            .route(&format!("/{}", Self::TABLE_NAME), get(list_post_handler))
            .route(
                &format!("/{}/{{id}}", Self::TABLE_NAME),
                get(get_post_handler),
            )
            .route(
                &format!("/{}/{{id}}", Self::TABLE_NAME),
                delete(delete_category_handler::<Self>),
            )
            .route(
                &format!("/{}/{{id}}", Self::TABLE_NAME),
                patch(update_category_handler::<Self::UpdatePayload>)
            )
    }
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct UpdatePostPayload {
    pub title: Option<String>,

    #[ts(type = "number | null")]
    pub source: Option<Value>,
    #[serde(skip)]
    pub source_str: String,

    pub content: Option<Vec<Content>>,
    #[serde(skip)]
    pub content_str: String,

    #[ts(type = "number | null")]
    pub thumb: Option<Value>,
    #[serde(skip)]
    pub thumb_id: Option<FileMetaId>,

    pub comments: Option<Vec<Comment>>,
    #[serde(skip)]
    pub comments_str: String,

    pub updated: Option<DateTime<Utc>>,
    #[serde(skip)]
    updated_str: String,

    pub published: Option<DateTime<Utc>>,
    #[serde(skip)]
    published_str: String,

    #[ts(type = "number | null")]
    pub platform: Option<Value>,
    #[serde(skip)]
    platform_id: Option<PlatformId>,

    // Relations
    pub authors: Option<Vec<u32>>,
    pub collections: Option<Vec<u32>>,
    pub tags: Option<Vec<u32>>,
}

impl UpdateCategoryPayload for UpdatePostPayload {
    const TABLE_NAME: &'static str = "posts";

    fn update(&mut self) -> UpdateContext<'_> {
        let mut ctx = UpdateContext::default();
        if let Some(title) = &self.title {
            ctx.content.push((":title", title));
        }
        if let Some(source) = &self.source {
            self.source_str = match source {
                Value::Null => "".to_string(),
                Value::String(s) => s.clone(),
                _ => "".to_string(),
            };
            ctx.content.push((":source", &self.source_str));
        }
        if let Some(content) = &self.content {
            self.content_str = serde_json::to_string(content).unwrap_or_default();
            ctx.content.push((":content", &self.content_str));
        }
        if let Some(thumb) = &self.thumb {
            ctx.content.push((
                ":thumb",
                match thumb {
                    Value::Null => &rusqlite::types::Null,
                    Value::Number(num) => {
                        self.thumb_id = num.as_u64().map(|n| FileMetaId(n as u32));
                        match self.thumb_id.as_ref() {
                            Some(id) => id,
                            None => &rusqlite::types::Null,
                        }
                    }
                    _ => &rusqlite::types::Null,
                },
            ));
        }
        if let Some(comments) = &self.comments {
            self.comments_str = serde_json::to_string(comments).unwrap_or_default();
            ctx.content.push((":comments", &self.comments_str));
        }
        if let Some(updated) = &self.updated {
            self.updated_str = updated.to_rfc3339();
            ctx.content.push((":updated", &self.updated_str));
        }
        if let Some(published) = &self.published {
            self.published_str = published.to_rfc3339();
            ctx.content.push((":published", &self.published_str));
        }
        if let Some(platform) = &self.platform {
            ctx.content.push((
                ":platform",
                match platform {
                    Value::Null => &rusqlite::types::Null,
                    Value::Number(num) => {
                        self.platform_id = num.as_u64().map(|n| PlatformId(n as u32));
                        match self.platform_id.as_ref() {
                            Some(id) => id,
                            None => &rusqlite::types::Null,
                        }
                    }
                    _ => &rusqlite::types::Null,
                },
            ));
        }

        // Relations
        if let Some(collections) = &self.collections {
            ctx.relations.push(RelationPayload {
                table: "collection_posts",
                field: "collection",
                related: "post",
                ids: collections.clone()
            });
        }

        if let Some(authors) = &self.authors {
            ctx.relations.push(RelationPayload {
                table: "author_posts",
                field: "author",
                related: "post",
                ids: authors.clone()
            });
        }

        if let Some(tags) = &self.tags {
            ctx.relations.push(RelationPayload {
                table: "post_tags",
                field: "tag",
                related: "post",
                ids: tags.clone()
            });
        }
        ctx
    }
}
