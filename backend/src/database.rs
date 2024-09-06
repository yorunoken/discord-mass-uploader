use std::fs::File;

use sqlx::{sqlite::SqlitePool, Executor};

pub async fn create_pool() -> SqlitePool {
    let db_path = "data.db";

    match File::open(db_path) {
        Err(_) => {
            println!("File not found. Creating a new file.");
            File::create(db_path).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            });
        }
        _ => {}
    }

    let database_url = format!("sqlite://{}", db_path);
    let db = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to the database.");

    db.execute(
        "CREATE TABLE IF NOT EXISTS files (thread_id TEXT NOT NULL, file_name TEXT NOT NULL);",
    )
    .await
    .unwrap_or_else(|error| {
        panic!("Problem creating `files` table: {:?}", error);
    });

    db
}
