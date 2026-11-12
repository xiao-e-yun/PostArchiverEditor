use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart, Path, State},
    http::StatusCode,
    routing::{delete, put},
};
use post_archiver::{FileMetaId, PostId, importer::UnsyncFileMeta, manager::PostArchiverBindExt};
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
    // 0.6 imports files by path: spool the uploads to a temp dir, then import them together.
    let dir = match tempfile::tempdir() {
        Ok(dir) => dir,
        Err(err) => {
            error!("failed to create temp dir: {err}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    let mut files = Vec::new();
    loop {
        let field = match multipart.next_field().await {
            Ok(Some(field)) => field,
            Ok(None) => break,
            Err(err) => {
                warn!("failed to read multipart form data: {err}");
                return StatusCode::BAD_REQUEST;
            }
        };

        let Some(filename) = field.file_name().and_then(plain_filename) else {
            warn!(
                "invalid filename in multipart form data: {:?}",
                field.file_name()
            );
            return StatusCode::BAD_REQUEST;
        };
        let mime = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();

        let data = match field.bytes().await {
            Ok(data) => data,
            Err(err) => {
                warn!("failed to read multipart form data: {err}");
                return StatusCode::BAD_REQUEST;
            }
        };

        let source = dir.path().join(files.len().to_string());
        if let Err(err) = tokio::fs::write(&source, &data).await {
            error!("failed to spool upload: {err}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
        files.push(UnsyncFileMeta::new(filename, mime, source));
    }

    let mut manager = state.manager();
    let mut importer = manager.importer();
    let result = (|| -> post_archiver::error::Result<()> {
        let mut sp = importer.savepoint()?;
        for file in files {
            sp.import_file_meta(id, file)?;
        }
        sp.commit()
    })();

    match result {
        Ok(()) => StatusCode::NO_CONTENT,
        Err(err) => {
            error!("failed to import file meta: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

/// Accept only a plain file name: no directories, `.` or `..`.
fn plain_filename(name: &str) -> Option<String> {
    let file_name = std::path::Path::new(name).file_name()?.to_str()?;
    (file_name == name && !name.contains('\\')).then(|| name.to_string())
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
