use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, patch},
};
use chrono::{DateTime, Utc};
use post_archiver::{Alias, Author, AuthorId, FileMetaId, PlatformId};
use rusqlite::Row;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::api::{
    AppState,
    category::{delete_category_handler, get_category_handler, list_category_handler, update_category_handler},
    relation::{RequireRelations, WithRelations},
};

use super::{Category, UpdateCategoryPayload};

impl RequireRelations for Author {
    fn file_metas(&self) -> Vec<post_archiver::FileMetaId> {
        self.thumb.into_iter().collect()
    }
}

impl Category for Author {
    type Id = AuthorId;
    type UpdatePayload = UpdateAuthorPayload;
    const TABLE_NAME: &'static str = "authors";

    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Author::from_row(row)
    }

    fn wrap_category_route(router: Router<AppState>) -> Router<AppState> {
        router
            .route(
                &format!("/{}", Self::TABLE_NAME),
                get(list_category_handler::<Self>),
            )
            .route(
                &format!("/{}/{{id}}", Self::TABLE_NAME),
                get(get_category_handler::<Self>),
            )
            .route(
                &format!("/{}/{{id}}", Self::TABLE_NAME),
                delete(delete_category_handler::<Self>),
            )
            .route(
                &format!("/{}/{{id}}", Self::TABLE_NAME),
                patch(update_category_handler::<Self::UpdatePayload>),
            )
            .route(
                &format!("/{}/{{id}}/aliases", Self::TABLE_NAME),
                get(author_aliases_handler),
            )
    }
}

pub async fn author_aliases_handler(
    State(state): State<AppState>,
    Path(id): Path<AuthorId>,
) -> Result<Json<WithRelations<Vec<Alias>>>, StatusCode> {
    let manager = &state.manager();
    let list = manager
        .list_author_aliases(id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    WithRelations::new(manager, list)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .map(Json::from)
}

impl RequireRelations for Alias {
    fn platforms(&self) -> Vec<PlatformId> {
        vec![self.platform]
    }
}

#[derive(Debug, Deserialize, Serialize, TS)]
pub struct UpdateAuthorPayload {
    pub name: Option<String>,
    #[ts(type = "number | null")]
    pub thumb: Option<serde_json::Value>,
    #[serde(skip)]
    pub thumb_id: Option<FileMetaId>,
    pub updated: Option<DateTime<Utc>>,
    #[serde(skip)]
    pub updated_str: String,
    pub aliases: Option<Vec<String>>,
}

impl UpdateCategoryPayload for UpdateAuthorPayload {
    const TABLE_NAME: &'static str = "authors";

    fn update(&mut self) -> super::UpdateContext<'_> {
        let mut ctx = super::UpdateContext::default();
        if let Some(name) = &self.name {
            ctx.content.push((":name", name));
        }
        if let Some(thumb) = &self.thumb {
            ctx.content.push((
                ":thumb",
                match thumb {
                    serde_json::Value::Null => &rusqlite::types::Null,
                    serde_json::Value::Number(num) => {
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
        if let Some(updated) = &self.updated {
            self.updated_str = updated.to_rfc3339();
            ctx.content.push((":updated", &self.updated_str));
        }
        ctx
    }
}
