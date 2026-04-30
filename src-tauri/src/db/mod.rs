use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};


pub async fn init_db(app_handle: &AppHandle) -> Result<SqlitePool, Box<dyn std::error::Error + Send + Sync>> {
    let app_dir = app_handle.path().app_data_dir()?;
    
    // Create the app data directory if it doesn't exist
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)?;
    }

    let db_path = app_dir.join("fieldstack.db");
    // Connect options with WAL mode and Foreign Keys
    let options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .foreign_keys(true)
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal);

    let pool = SqlitePool::connect_with(options).await?;

    // Optimization PRAGMAs for M1 (approx 2MB RAM cache)
    sqlx::query("PRAGMA cache_size = -2000;").execute(&pool).await?;
    sqlx::query("PRAGMA temp_store = MEMORY;").execute(&pool).await?;
    sqlx::query("PRAGMA mmap_size = 30000000000;").execute(&pool).await?;

    // Run migrations (schema.sql)
    let schema = include_str!("schema.sql");
    sqlx::query(schema).execute(&pool).await?;

    // Create proofs directory
    let proofs_dir = app_dir.join("proofs");
    if !proofs_dir.exists() {
        fs::create_dir_all(&proofs_dir)?;
    }

    Ok(pool)
}

pub fn get_proofs_path(app_handle: &AppHandle, job_id: &str) -> PathBuf {
    let app_dir = app_handle.path().app_data_dir().expect("Failed to get app data directory");
    app_dir.join("proofs").join(job_id)
}
