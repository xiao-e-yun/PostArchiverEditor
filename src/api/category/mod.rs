pub mod author;
pub mod collection;
pub mod file_meta;
pub mod platform;
pub mod post;
pub mod tag;

use std::{fmt::Debug, hash::Hash};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, patch},
};
use axum_extra::extract::Query;
use post_archiver::manager::PostArchiverManager;
use rusqlite::{OptionalExtension, Row, ToSql};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tracing::warn;
use ts_rs::TS;

use super::{
    AppState,
    relation::{RequireRelations, WithRelations},
    utils::{ListResponse, Pagination},
};

pub trait Category: RequireRelations + Serialize + Debug + TS + Sized + 'static {
    type Id: From<u32> + Debug + Serialize + ToSql + Copy + Eq + Hash + Sync + Send + 'static;
    type UpdatePayload: UpdateCategoryPayload;
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
            .route(
                &format!("/{}/{{id}}", Self::TABLE_NAME),
                delete(delete_category_handler::<Self>),
            )
            .route(
                &format!("/{}/{{id}}", Self::TABLE_NAME),
                patch(update_category_handler::<Self::UpdatePayload>),
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
            (
                format!("WHERE {} LIKE concat('%',:search,'%')", Self::SEARCH_NAME),
                Some(search),
            )
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

async fn delete_category_handler<T: Category>(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<StatusCode, StatusCode> {
    let manager = &state.manager();
    let id: T::Id = id.into();

    match manager
        .conn()
        .execute(&format!("DELETE FROM {} WHERE id = ?", T::TABLE_NAME), [id])
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub trait UpdateCategoryPayload: DeserializeOwned + Debug + Send + Sync + 'static {
    const TABLE_NAME: &'static str;
    fn update(&mut self) -> UpdateContext<'_>;
}

#[derive(Default)]
pub struct UpdateContext<'a> {
    pub content: Vec<(&'static str, &'a dyn ToSql)>,
    pub relations: Vec<RelationPayload>,
}

#[derive(Debug)]
pub struct RelationPayload {
    pub table: &'static str,
    pub field: &'static str,
    pub related: &'static str,
    pub ids: Vec<u32>,
}

async fn update_category_handler<T>(
    Path(id): Path<u32>,
    State(state): State<AppState>,
    Json(mut payload): Json<T>,
) -> Result<(), StatusCode>
where
    T: UpdateCategoryPayload + DeserializeOwned + Debug + 'static,
{
    let ctx = payload.update();
    if ctx.content.is_empty() && ctx.relations.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut manager = state.manager();
    let tx = manager
        .transaction()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !ctx.content.is_empty() {
        let set_clause = ctx
            .content
            .iter()
            .map(|(k, _)| format!("{} = {}", &k[1..], k))
            .collect::<Vec<String>>()
            .join(", ");

        let mut stmt = tx
            .conn()
            .prepare(&format!(
                "UPDATE {} SET {} WHERE id = :id",
                T::TABLE_NAME,
                set_clause
            ))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let params: Vec<(&'static str, &dyn ToSql)> = ctx
            .content
            .iter()
            .chain(Some((":id", &id as &dyn ToSql)).iter())
            .cloned()
            .collect();

        stmt.execute(params.as_slice())
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    };

    if !ctx.relations.is_empty() {
        for relation in ctx.relations {
            let mut stmt = tx
                .conn()
                .prepare_cached(&format!(
                    "DELETE FROM {} WHERE {} = ? AND {} NOT IN (SELECT value FROM json_each(?))",
                    relation.table, relation.related, relation.field,
                ))
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            stmt.execute(rusqlite::params![
                id,
                serde_json::to_string(&relation.ids).unwrap()
            ])
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let mut stmt = tx
                .conn()
                .prepare_cached(&format!(
                    "INSERT OR IGNORE INTO {} ({}, {}) SELECT ?, value FROM json_each(?)",
                    relation.table, relation.related, relation.field,
                ))
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            stmt.execute(rusqlite::params![
                id,
                serde_json::to_string(&relation.ids).unwrap()
            ])
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
    }

    tx.commit().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
