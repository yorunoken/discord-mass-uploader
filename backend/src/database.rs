use sqlx::sqlite::SqlitePool;

pub async fn create_pool() -> SqlitePool {
    let db_path = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set.");

    SqlitePool::connect(&db_path)
        .await
        .expect("Failed to connect to the database.")
}
