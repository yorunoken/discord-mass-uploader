use std::sync::Arc;

use serenity::all::Http;
use sqlx::SqlitePool;
use warp::{body::json, Filter, Rejection, Reply};

use crate::api::{add_file, delete_file, download, files, upload, upload_progress, AppState};

pub fn routes(
    pool: SqlitePool,
    client: Arc<Http>,
    state: Arc<AppState>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // GET routes
    let get_files = warp::path!("api" / "files")
        .and(warp::get())
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and(with_db(pool.clone()))
        .and_then(files);

    let download_file = warp::path!("api" / "download")
        .and(warp::get())
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and(with_client(client.clone()))
        .and_then(download);

    let upload_progress = warp::path!("api" / "upload" / "progress")
        .and(warp::get())
        .and(with_state(state.clone()))
        .and_then(upload_progress);

    // POST routes
    let post_upload = warp::path!("api" / "upload")
        .and(warp::post())
        .and(json())
        .and(with_client(client.clone()))
        .and(with_state(state.clone()))
        .and_then(upload);

    let post_files = warp::path!("api" / "database" / "file")
        .and(warp::post())
        .and(json())
        .and(with_db(pool.clone()))
        .and_then(add_file);

    let delete_files = warp::path!("api" / "database" / "file" / "delete")
        .and(warp::post())
        .and(json())
        .and(with_db(pool.clone()))
        .and_then(delete_file);

    get_files
        .or(post_upload)
        .or(upload_progress)
        .or(post_files)
        .or(download_file)
        .or(delete_files)
}

fn with_db(
    pool: SqlitePool,
) -> impl Filter<Extract = (SqlitePool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

fn with_client(
    client: Arc<Http>,
) -> impl Filter<Extract = (Arc<Http>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}

fn with_state(
    state: Arc<AppState>,
) -> impl Filter<Extract = (Arc<AppState>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}
