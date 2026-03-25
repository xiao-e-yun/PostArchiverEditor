use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use chrono::{DateTime, Utc};
use optional_field::Field;
use post_archiver::{
    Alias, Author, AuthorId, FileMetaId, PlatformId,
    manager::{PostArchiverManager, UpdateAuthor},
    query::{Countable, Paginate, Query, SortDir, Sortable, Totalled, author::AuthorSort},
};
use serde::{Deserialize, Serialize};

use crate::api::{
    AppState,
    category::{
        delete_category_handler, get_category_handler, list_category_handler,
        list_category_posts_handler, update_category_handler,
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
        q.sort(AuthorSort::Id, SortDir::Desc)
            .pagination(pagination.limit(), pagination.page())
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
                &format!("/{}/{{id}}/posts", Self::ROUTE),
                get(list_category_posts_handler::<Self>),
            )
            .route(
                &format!("/{}/{{id}}/aliases", Self::ROUTE),
                get(author_aliases_handler).patch(update_author_aliases_handler),
            )
    }

    fn filter_posts<T>(
        mut query: post_archiver::query::post::PostQuery<T>,
        id: Self::Id,
    ) -> post_archiver::query::post::PostQuery<T> {
        query.authors.insert(id);
        query
    }
}

pub async fn author_aliases_handler(
    State(state): State<AppState>,
    Path(id): Path<AuthorId>,
) -> Result<Json<WithRelations<Totalled<Vec<Alias>>>>, StatusCode> {
    let manager = &state.manager();
    let list = manager
        .bind(id)
        .list_aliases()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    WithRelations::new(
        manager,
        Totalled {
            total: list.len() as u64,
            items: list,
        },
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    .map(Json::from)
}

impl RequireRelations for Alias {
    fn platforms(&self) -> Vec<PlatformId> {
        vec![self.platform]
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateAuthorPayload {
    pub name: Option<String>,
    pub thumb: Field<FileMetaId>,
    pub updated: Option<DateTime<Utc>>,
}

impl UpdateCategoryPayload<AuthorId> for UpdateAuthorPayload {
    fn apply(
        self,
        manager: &PostArchiverManager,
        id: AuthorId,
    ) -> post_archiver::error::Result<()> {
        let mut update = UpdateAuthor::default();
        if let Some(name) = self.name {
            update = update.name(name);
        }
        if let Field::Present(thumb) = self.thumb {
            update = update.thumb(thumb);
        }
        if let Some(updated) = self.updated {
            update = update.updated(updated);
        }
        manager.bind(id).update(update)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateAuthorAliasesPayload {
    items: Vec<Alias>,
}

async fn update_author_aliases_handler(
    Path(id): Path<AuthorId>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateAuthorAliasesPayload>,
) -> Result<StatusCode, StatusCode> {
    let manager = state.manager();

    let bound = manager.bind(id);

    let new_aliases = payload.items;
    let current = manager
        .bind(id)
        .list_aliases()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let to_remove: Vec<_> = current
        .iter()
        .filter(|&a| !new_aliases.contains(a))
        .cloned()
        .map(|a| (a.source, a.platform))
        .collect();
    let to_add: Vec<_> = new_aliases
        .iter()
        .filter(|&a| !current.contains(a))
        .cloned()
        .map(|a| (a.source, a.platform, a.link))
        .collect();
    bound
        .remove_aliases(&to_remove)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    bound
        .add_aliases(to_add)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
