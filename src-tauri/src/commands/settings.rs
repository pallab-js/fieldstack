use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct AppConfig {
    pub key: String,
    pub value: String,
}

/// Returns all key-value pairs from app_config.
#[tauri::command]
pub async fn get_app_config(pool: State<'_, SqlitePool>) -> Result<Vec<AppConfig>, String> {
    sqlx::query_as::<_, AppConfig>("SELECT key, value FROM app_config ORDER BY key")
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())
}

/// Upserts a single key-value pair in app_config.
#[tauri::command]
pub async fn set_app_config(
    pool: State<'_, SqlitePool>,
    key: String,
    value: String,
) -> Result<(), String> {
    sqlx::query(
        "INSERT INTO app_config (key, value) VALUES (?, ?)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
    )
    .bind(&key)
    .bind(&value)
    .execute(&*pool)
    .await
    .map(|_| ())
    .map_err(|e| e.to_string())
}

/// Deletes all user data (jobs, proofs, audit_log, drafts) and resets the job counter.
/// Companies, people, and app_config are preserved.
#[tauri::command]
pub async fn reset_job_data(pool: State<'_, SqlitePool>) -> Result<(), String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    sqlx::query("DELETE FROM audit_log").execute(&mut *tx).await.map_err(|e| e.to_string())?;
    sqlx::query("DELETE FROM proofs").execute(&mut *tx).await.map_err(|e| e.to_string())?;
    sqlx::query("DELETE FROM job_drafts").execute(&mut *tx).await.map_err(|e| e.to_string())?;
    sqlx::query("DELETE FROM jobs").execute(&mut *tx).await.map_err(|e| e.to_string())?;
    sqlx::query("UPDATE job_counter SET last_val = 0").execute(&mut *tx).await.map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())
}
