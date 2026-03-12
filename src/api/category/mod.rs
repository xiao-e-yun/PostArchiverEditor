pub mod author;
pub mod collection;
pub mod platform;
pub mod post;
pub mod tag;

use std::{fmt::Debug, hash::Hash};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use axum_extra::extract::Query;
use post_archiver::{
    manager::{BindableId, PostArchiverManager},
    query::{Countable, Paginate, Totalled, post::PostQuery},
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use ts_rs::TS;

use super::{
    AppState,
    post::PostShortResponse,
    relation::{RequireRelations, WithRelations},
    utils::Pagination,
};

pub trait Category: RequireRelations + Serialize + Debug + TS + Sized + 'static {
    type Id: From<u32> + BindableId + Debug + Serialize + Copy + Eq + Hash + Sync + Send + 'static;
    type UpdatePayload: UpdateCategoryPayload<Self::Id>;

    const ROUTE: &'static str;

    fn list_query(
        manager: &PostArchiverManager,
        pagination: &Pagination,
        search: &str,
    ) -> post_archiver::error::Result<Totalled<Vec<Self>>>;

    fn get_single(
        manager: &PostArchiverManager,
        id: Self::Id,
    ) -> post_archiver::error::Result<Option<Self>>;

    fn delete_entity(
        manager: &PostArchiverManager,
        id: Self::Id,
    ) -> post_archiver::error::Result<()>;

    fn filter_posts<T>(query: PostQuery<T>, id: Self::Id) -> PostQuery<T>;

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
) -> Result<Json<WithRelations<Totalled<Vec<T>>>>, StatusCode> {
    let manager = state.manager();
    let result = T::list_query(&manager, &pagination, &filter.search)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    WithRelations::new(&manager, result)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .map(Json::from)
}

async fn get_category_handler<T: Category>(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<Json<WithRelations<T>>, StatusCode> {
    let manager = state.manager();
    let id: T::Id = id.into();

    let item = T::get_single(&manager, id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    WithRelations::new(&manager, item)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .map(Json::from)
}

async fn delete_category_handler<T: Category>(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<StatusCode, StatusCode> {
    let manager = state.manager();
    let id: T::Id = id.into();

    T::delete_entity(&manager, id)
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn update_category_handler<T: Category>(
    Path(id): Path<u32>,
    State(state): State<AppState>,
    Json(payload): Json<T::UpdatePayload>,
) -> Result<StatusCode, StatusCode> {
    let manager = state.manager();
    let id: T::Id = id.into();

    payload
        .apply(&manager, id)
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub trait UpdateCategoryPayload<Id>: DeserializeOwned + Debug + Send + Sync + 'static {
    fn apply(self, manager: &PostArchiverManager, id: Id) -> post_archiver::error::Result<()>;
}

async fn list_category_posts_handler<T: Category>(
    Path(id): Path<u32>,
    Query(pagination): Query<Pagination>,
    State(state): State<AppState>,
) -> Result<Json<WithRelations<Totalled<Vec<PostShortResponse>>>>, StatusCode> {
    let manager = state.manager();
    let id: T::Id = id.into();

    let query = T::filter_posts(manager.posts(), id);

    use post_archiver::query::Query;
    let result = query
        .pagination(pagination.limit(), pagination.page())
        .with_total()
        .query::<PostShortResponse>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    WithRelations::new(&manager, result)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .map(Json::from)
}
