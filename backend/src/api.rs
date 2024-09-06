use base64::{engine::general_purpose, Engine as _};
use serenity::all::{ChannelId, CreateAttachment, CreateMessage, CreateThread, Http};
use std::{collections::HashMap, fmt::Write, path::Path, sync::Arc};

use futures::StreamExt;
use tokio_stream::wrappers::ReceiverStream;

use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use warp::{http::StatusCode, reject::Rejection, reply::Reply, sse::Event};

use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
    sync::{
        mpsc::{self, Sender},
        Mutex,
    },
};

use crate::{models::Files, utils::download::download_file};

pub struct AppState {
    progress_sender: Mutex<Option<Sender<f64>>>,
}

impl AppState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            progress_sender: Mutex::new(None),
        })
    }
}

// GET
pub async fn files(
    query: HashMap<String, String>,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    let mut sql_query = String::from("SELECT * FROM files");

    if !query.is_empty() {
        let mut conditions: Vec<String> = Vec::new();
        for (key, value) in query {
            conditions.push(format!("{} = '{}'", key, value))
        }
        let where_clause = conditions.join(" AND ");

        let _ = write!(sql_query, " WHERE {}", where_clause);
    }

    let rows = sqlx::query(&sql_query)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            eprintln!("failed to get query: {e}");
            warp::reject::not_found()
        })?;

    let files: Vec<Files> = rows
        .iter()
        .map(|row| Files {
            file_name: row.get("file_name"),
            thread_id: row.get("thread_id"),
        })
        .collect();

    Ok(warp::reply::json(&files))
}

pub async fn upload_progress(state: Arc<AppState>) -> Result<impl Reply, Rejection> {
    let (tx, rx) = tokio::sync::mpsc::channel(100);

    *state.progress_sender.lock().await = Some(tx);

    let stream = ReceiverStream::new(rx);
    let event_stream =
        stream.map(|progress| Ok::<_, warp::Error>(Event::default().data(progress.to_string())));

    Ok(warp::sse::reply(event_stream))
}

pub async fn download(
    query: HashMap<String, String>,
    client: Arc<Http>,
) -> Result<Box<dyn Reply>, Rejection> {
    let thread_id = match query.get("thread_id") {
        Some(s) => s.to_string(),
        None => {
            return Ok(Box::new(warp::reply::with_status(
                warp::reply::json(&ErrorResponse {
                    error: String::from("`thread_id` must be a valid query"),
                }),
                StatusCode::BAD_REQUEST,
            )))
        }
    };

    let file_name = match query.get("file") {
        Some(s) => s.to_string(),
        None => {
            return Ok(Box::new(warp::reply::with_status(
                warp::reply::json(&ErrorResponse {
                    error: String::from("`thread_id` must be a valid query"),
                }),
                StatusCode::BAD_REQUEST,
            )))
        }
    };

    let (tx, mut rx) = mpsc::channel(100);

    download_file(thread_id, file_name, client, tx).await;

    let stream = async_stream::stream! {
        while let Some(progress) = rx.recv().await {
            yield Ok::<_, warp::Error>(Event::default().data(progress))
        }
    };

    Ok(Box::new(warp::sse::reply(stream)))
}

// POST
#[derive(Deserialize)]
pub struct UploadRequest {
    channel_id: String,
    file_path: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

#[derive(Serialize)]
pub struct SuccessResponse {
    file_name: String,
    thread_id: String,
}

const CHUNK_SIZE: usize = 18_874_368; // 18 MB
pub async fn upload(
    request: UploadRequest,
    client: Arc<Http>,
    state: Arc<AppState>,
) -> Result<impl Reply, Rejection> {
    let UploadRequest {
        channel_id,
        file_path,
    } = request;

    let file_name = Path::new(&file_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown")
        .to_string();

    let channel_id: u64 = match channel_id.parse() {
        Ok(ok) => ok,
        Err(why) => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&ErrorResponse {
                    error: format!("Cannot parse channel_id: {}", why),
                }),
                StatusCode::BAD_REQUEST,
            ));
        }
    };
    let channel_id = ChannelId::new(channel_id);

    // Create thread
    let thread = match client
        .create_thread(
            channel_id,
            &CreateThread::new(file_name.clone()).kind(serenity::all::ChannelType::PublicThread),
            None,
        )
        .await
    {
        Err(e) => {
            eprintln!("Cannot create thread: {}", e);
            return Ok(warp::reply::with_status(
                warp::reply::json(&ErrorResponse {
                    error: format!("Cannot create thread: {}", e),
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
        Ok(ok) => ok,
    };

    let progress_sender = state.progress_sender.lock().await.take();

    let thread_clone = thread.clone();
    let file_name_clone = file_name.clone();

    let file = match File::open(&file_path.clone()).await {
        Ok(file) => file,
        Err(e) => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&ErrorResponse {
                    error: format!("Cannot read file: {}", e),
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    let metadata = match file.metadata().await {
        Ok(o) => o,
        Err(e) => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&ErrorResponse {
                    error: format!("Failed to fetch file metadata: {}", e),
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    let file_size = metadata.len();

    if let Some(sender) = progress_sender {
        let mut reader = BufReader::new(file);
        let mut buffer = vec![0; CHUNK_SIZE];
        let mut index = 0;
        let mut total_bytes_processed = 0;

        tokio::spawn(async move {
            loop {
                let bytes_read = reader.read(&mut buffer).await.unwrap();
                if bytes_read == 0 {
                    break;
                }
                total_bytes_processed += bytes_read;

                let base64_chunk = general_purpose::STANDARD.encode(&buffer[..bytes_read]);

                let attachment = CreateAttachment::bytes(
                    base64_chunk.as_bytes(),
                    format!("{}_{}.txt", file_name_clone, index),
                );

                if let Err(why) = thread_clone
                    .send_message(
                        &client,
                        CreateMessage::new()
                            .content(format!("{}_{}.txt", file_name_clone, index))
                            .add_file(attachment),
                    )
                    .await
                {
                    eprintln!("Cannot send message to thread: {}", why);
                    let _ = sender.send(0.0).await;
                    return;
                }

                let progress = (total_bytes_processed as f64 / file_size as f64) * 100.0;
                println!("progress: {}", progress);
                let _ = sender.send(progress).await;

                index += 1;
            }
        });
    }

    Ok(warp::reply::with_status(
        warp::reply::json(&SuccessResponse {
            file_name: file_name.to_string(),
            thread_id: thread.id.to_string(),
        }),
        StatusCode::OK,
    ))
}

#[derive(Deserialize)]
pub struct FileRequest {
    file_name: String,
    thread_id: String,
}

pub async fn add_file(request: FileRequest, pool: SqlitePool) -> Result<impl Reply, Rejection> {
    let FileRequest {
        file_name,
        thread_id,
    } = request;

    sqlx::query!(
        "INSERT INTO files (thread_id, file_name) VALUES (?, ?)",
        thread_id,
        file_name,
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to insert feedback: {:?}", e);
        warp::reject::reject()
    })?;

    Ok(warp::reply::with_status(
        "Added file to database",
        StatusCode::CREATED,
    ))
}

pub async fn delete_file(request: FileRequest, pool: SqlitePool) -> Result<impl Reply, Rejection> {
    let FileRequest {
        file_name,
        thread_id,
    } = request;

    sqlx::query!(
        "DELETE FROM files WHERE thread_id = ? AND file_name = ?",
        thread_id,
        file_name,
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to delete file: {:?}", e);
        warp::reject::reject()
    })?;

    Ok(warp::reply::with_status(
        "File deleted from database",
        StatusCode::OK,
    ))
}
