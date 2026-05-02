use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer, Registry};

/// Initialize structured logging with file output.
/// Called at app startup before any other subsystem.
/// Returns the path to the log file for reference.
pub fn init_logging(app_handle: &AppHandle) -> PathBuf {
    let log_dir = app_handle
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from("."));
    
    if !log_dir.exists() {
        let _ = fs::create_dir_all(&log_dir);
    }

    // Rotating log: new file each day, keep 7 days
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        &log_dir,
        "fieldstack.log",
    );

    // Non-blocking writer (flushes in background)
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // Console layer: human-readable for development
    let console_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("fieldstack_lib=info,tauri=warn"));

    // File layer: JSON structured for machine parsing
    let file_filter = EnvFilter::new("fieldstack_lib=info");

    Registry::default()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_level(true)
                .with_ansi(true)
                .with_filter(console_filter),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_writer(non_blocking)
                .with_filter(file_filter),
        )
        .init();

    tracing::info!(app_dir = %log_dir.display(), "Logging initialized");

    log_dir.join("fieldstack.log")
}

/// Write a crash marker to the log file. Called on startup to detect
/// if the previous run ended abnormally (crash marker was never cleared).
pub fn check_previous_crash(app_handle: &AppHandle) -> Option<String> {
    let log_dir = app_handle
        .path()
        .app_data_dir()
        .ok()?;
    
    let crash_marker = log_dir.join("fieldstack.crash");
    if crash_marker.exists() {
        let content = fs::read_to_string(&crash_marker).ok()?;
        // Clear the marker after reading
        let _ = fs::remove_file(&crash_marker);
        if !content.trim().is_empty() {
            return Some(content);
        }
    }
    None
}

/// Set a crash marker for the current session. If the app crashes,
/// the marker will be detected on next launch.
pub fn set_crash_marker(app_handle: &AppHandle, context: &str) {
    let log_dir = app_handle
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from("."));
    
    let crash_marker = log_dir.join("fieldstack.crash");
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&crash_marker)
    {
        let _ = writeln!(file, "Crash at {} — {}", chrono::Utc::now(), context);
    }
}

/// Clear the crash marker (called during graceful shutdown).
pub fn clear_crash_marker(app_handle: &AppHandle) {
    let log_dir = app_handle
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from("."));
    
    let crash_marker = log_dir.join("fieldstack.crash");
    let _ = fs::remove_file(&crash_marker);
}

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
    // 2GB mmap — safe for 8GB RAM machines
    sqlx::query("PRAGMA mmap_size = 2147483648;").execute(&pool).await?;

    // Run pending migrations from src-tauri/migrations/
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Create proofs directory
    let proofs_dir = app_dir.join("proofs");
    if !proofs_dir.exists() {
        fs::create_dir_all(&proofs_dir)?;
    }

    tracing::info!(db_path = %db_path.display(), "Database initialized");

    Ok(pool)
}

pub fn get_proofs_path(app_handle: &AppHandle, job_id: &str) -> PathBuf {
    let app_dir = app_handle.path().app_data_dir().expect("Failed to get app data directory");
    app_dir.join("proofs").join(job_id)
}
