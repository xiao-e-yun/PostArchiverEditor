use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart, Path, State},
    http::StatusCode,
    routing::{delete, put},
};
use post_archiver::{FileMetaId, PostId, importer::UnsyncFileMeta};
use tracing::{error, warn};

use super::AppState;

pub fn wrap_file_route(router: Router<AppState>) -> Router<AppState> {
    const SIZE: usize = 128 * 1024 * 1024; // 128 MB

    router
        .route("/posts/{id}/files", put(upload_file_handler))
        .layer(DefaultBodyLimit::max(SIZE))
        .route("/files/{id}", delete(remove_file_handler))
}

async fn upload_file_handler(
    Path(id): Path<PostId>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> StatusCode {
    while let Some(field) = multipart.next_field().await.unwrap() {
        // !WARNING: this is not safe, the filename can be something like "../../etc/passwd"
        // TODO: sanitize the filename
        let Some(filename) = field.file_name().map(|s| s.to_string()) else {
            warn!("invalid filename in multipart form data");
            continue;
        };
        let mime = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();

        let data = match field.bytes().await {
            Ok(data) => data.to_vec(),
            Err(err) => {
                warn!("failed to read multipart form data: {err}");
                continue;
            }
        };

        let file_meta = UnsyncFileMeta::new(filename, mime, data);
        if let Err(err) = state
            .manager()
            .import_file_meta_with_content(id, &file_meta)
        {
            error!("failed to import file meta: {err}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        };
    }
    StatusCode::NO_CONTENT
}

async fn remove_file_handler(
    Path(id): Path<FileMetaId>,
    State(state): State<AppState>,
) -> StatusCode {
    let manager = state.manager();
    let binded = manager.bind(id);
    let Ok(file_meta) = binded.value() else {
        warn!("file meta not found: {id}");
        return StatusCode::NOT_FOUND;
    };
    match binded.delete() {
        Ok(_) => {
            let path = manager.path.join(file_meta.path());
            if let Err(err) = std::fs::remove_file(path) {
                error!("failed to delete file: {err}");
            }
            StatusCode::NO_CONTENT
        }
        Err(err) => {
            warn!("failed to delete file meta: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
