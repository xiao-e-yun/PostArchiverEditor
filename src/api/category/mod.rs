pub mod author;
pub mod collection;
pub mod platform;
pub mod tag;
pub mod post;
pub mod file_meta;

use std::{fmt::Debug, hash::Hash};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use axum_extra::extract::Query;
use post_archiver::manager::PostArchiverManager;
use rusqlite::{OptionalExtension, Row, ToSql};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{
    AppState,
    relation::{RequireRelations, WithRelations},
    utils::{ListResponse, Pagination},
};

pub trait Category: RequireRelations + Serialize + Debug + TS + Sized + 'static {
    type Id: From<u32> + Debug + Serialize + ToSql + Copy + Eq + Hash + Sync + Send + 'static;
    const TABLE_NAME: &'static str;
    const SEARCH_NAME: &'static str = "name";
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error>;

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
    }

    fn list(
        manager: &PostArchiverManager,
        pagination: Pagination,
        search: String,
    ) -> Result<Vec<Self>, rusqlite::Error> {
        let params = pagination.params();
        let (filter, search_params) = if search.is_empty() {
            (String::new(), None)
        } else {
            (format!("WHERE {} LIKE concat('%',:search,'%')", Self::SEARCH_NAME), Some(search))
        };

        let mut stmt = manager.conn().prepare_cached(&format!(
            "SELECT * FROM {} {} ORDER BY id DESC LIMIT :limit OFFSET :offset",
            Self::TABLE_NAME,
            filter,
        ))?;

        let params = params
            .iter()
            .map(|(k, v)| (*k, v as &dyn ToSql))
            .chain(search_params.as_ref().map(|s| (":search", s as &dyn ToSql)))
            .collect::<Vec<(&'static str, &dyn ToSql)>>();
        let list = stmt.query_map(params.as_slice(), Self::from_row)?;

        list.collect()
    }

    fn get(
        manager: &PostArchiverManager,
        id: Self::Id,
    ) -> Result<Option<WithRelations<Self>>, rusqlite::Error> {
        let mut stmt = manager
            .conn()
            .prepare_cached(&format!("SELECT * FROM {} WHERE id = ?", Self::TABLE_NAME))?;

        stmt.query_row([id], Self::from_row)
            .optional()?
            .map(|c| WithRelations::new(manager, c))
            .transpose()
    }
}

#[derive(Debug, Deserialize)]
pub struct Filter {
    #[serde(default)]
    pub search: String,
}

async fn list_category_handler<T: Category>(
    Query(filter): Query<Filter>,
    Query(pagination): Query<Pagination>,
    State(state): State<AppState>,
) -> Result<Json<WithRelations<ListResponse<T>>>, StatusCode> {
    let manager = &state.manager();
    let list = T::list(manager, pagination, filter.search.clone())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    WithRelations::new(manager, ListResponse { list })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .map(Json::from)
}

async fn get_category_handler<T: Category>(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<Json<WithRelations<T>>, StatusCode> {
    let manager = &state.manager();
    let id: T::Id = id.into();

    T::get(manager, id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)
        .map(Json::from)
}
