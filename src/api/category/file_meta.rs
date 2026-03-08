use std::collections::HashMap;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use post_archiver::{
    FileMeta, FileMetaId,
    manager::{PostArchiverManager, UpdateFileMeta},
    query::FromQuery,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;

use crate::api::{
    AppState,
    category::UpdateCategoryPayload,
    relation::{RequireRelations, WithRelations},
    utils::Pagination,
};
use axum_extra::extract::Query as ExQuery;

impl RequireRelations for FileMeta {}

pub fn file_meta_routes(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/file_metas", get(list_file_meta_handler))
        .route(
            "/file_metas/{id}",
            get(get_file_meta_handler)
                .delete(delete_file_meta_handler)
                .patch(update_file_meta_handler),
        )
}

async fn list_file_meta_handler(
    ExQuery(pagination): ExQuery<Pagination>,
    State(state): State<AppState>,
) -> Result<Json<WithRelations<Vec<FileMeta>>>, StatusCode> {
    let manager = state.manager();
    let limit = pagination.limit() as i64;
    let offset = (pagination.page() * pagination.limit()) as i64;
    let list: Vec<FileMeta> = {
        let mut stmt = manager
            .conn()
            .prepare_cached(
                "SELECT * FROM file_metas ORDER BY id LIMIT ? OFFSET ?",
            )
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        stmt.query_map([limit, offset], FileMeta::from_row)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .filter_map(|r| r.ok())
            .collect()
    };
    WithRelations::new(&manager, list)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .map(Json::from)
}

async fn get_file_meta_handler(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<Json<WithRelations<FileMeta>>, StatusCode> {
    let manager = state.manager();
    let id = FileMetaId::from(id);
    let item = manager
        .get_file_meta(id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    WithRelations::new(&manager, item)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .map(Json::from)
}

async fn delete_file_meta_handler(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<StatusCode, StatusCode> {
    let manager = state.manager();
    manager
        .bind(FileMetaId::from(id))
        .delete()
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn update_file_meta_handler(
    Path(id): Path<u32>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateFileMetaPayload>,
) -> Result<StatusCode, StatusCode> {
    let manager = state.manager();
    payload
        .apply(&manager, FileMetaId::from(id))
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct UpdateFileMetaPayload {
    pub mime: Option<String>,
    #[ts(type = "Record<string, any>")]
    pub extra: Option<HashMap<String, Value>>,
}

impl UpdateCategoryPayload<FileMetaId> for UpdateFileMetaPayload {
    fn apply(self, manager: &PostArchiverManager, id: FileMetaId) -> post_archiver::error::Result<()> {
        let mut update = UpdateFileMeta::default();
        if let Some(mime) = self.mime {
            update = update.mime(mime);
        }
        if let Some(extra) = self.extra {
            update = update.extra(extra);
        }
        manager.bind(id).update(update)
    }
}
