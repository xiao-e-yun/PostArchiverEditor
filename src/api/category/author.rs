use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use chrono::{DateTime, Utc};
use post_archiver::{
    Alias, Author, AuthorId, FileMetaId, PlatformId,
    manager::{PostArchiverManager, UpdateAuthor},
    query::{Totalled, Paginate, Countable, Query},
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::api::{
    AppState,
    category::{
        delete_category_handler, get_category_handler, list_category_handler,
        update_category_handler,
    },
    relation::{RequireRelations, WithRelations},
    utils::Pagination,
};

use super::{Category, UpdateCategoryPayload};

impl RequireRelations for Author {
    fn file_metas(&self) -> Vec<FileMetaId> {
        self.thumb.into_iter().collect()
    }
}

impl Category for Author {
    type Id = AuthorId;
    type UpdatePayload = UpdateAuthorPayload;

    const ROUTE: &'static str = "authors";

    fn list_query(
        manager: &PostArchiverManager,
        pagination: &Pagination,
        search: &str,
    ) -> post_archiver::error::Result<Totalled<Vec<Self>>> {
        let mut q = manager.authors();
        if !search.is_empty() {
            q.name.contains(search);
        }
        q.pagination(pagination.limit(), pagination.page())
            .with_total()
            .query::<Author>()
    }

    fn get_single(
        manager: &PostArchiverManager,
        id: Self::Id,
    ) -> post_archiver::error::Result<Option<Self>> {
        manager.get_author(id)
    }

    fn delete_entity(
        manager: &PostArchiverManager,
        id: Self::Id,
    ) -> post_archiver::error::Result<()> {
        manager.bind(id).delete()
    }

    fn wrap_category_route(router: Router<AppState>) -> Router<AppState> {
        router
            .route(
                &format!("/{}", Self::ROUTE),
                get(list_category_handler::<Self>),
            )
            .route(
                &format!("/{}/{{id}}", Self::ROUTE),
                get(get_category_handler::<Self>)
                    .delete(delete_category_handler::<Self>)
                    .patch(update_category_handler::<Self>),
            )
            .route(
                &format!("/{}/{{id}}/aliases", Self::ROUTE),
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
        .bind(id)
        .list_aliases()
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
    pub updated: Option<DateTime<Utc>>,
    pub aliases: Option<Vec<String>>,
}

impl UpdateCategoryPayload<AuthorId> for UpdateAuthorPayload {
    fn apply(self, manager: &PostArchiverManager, id: AuthorId) -> post_archiver::error::Result<()> {
        let mut update = UpdateAuthor::default();
        if let Some(name) = self.name {
            update = update.name(name);
        }
        if let Some(thumb) = self.thumb {
            update = update.thumb(match thumb {
                serde_json::Value::Null => None,
                serde_json::Value::Number(n) => n.as_u64().map(|n| FileMetaId(n as u32)),
                _ => None,
            });
        }
        if let Some(updated) = self.updated {
            update = update.updated(updated);
        }
        manager.bind(id).update(update)
    }
}
