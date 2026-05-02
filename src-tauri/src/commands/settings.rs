use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::{State, Manager};
use std::fs;

/// Allowlist of valid config keys. Prevents arbitrary key injection.
const ALLOWED_CONFIG_KEYS: &[&str] = &[
    "theme",
    "language",
    "default_company_id",
    "sidebar_collapsed",
    "items_per_page",
    "notification_sound",
];

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
        .map_err(|e| {
            tracing::error!(error = %e, "DB error in get_app_config");
            "Failed to load settings".to_string()
        })
}

/// Upserts a single key-value pair in app_config. Keys must be on the allowlist.
#[tauri::command]
pub async fn set_app_config(
    pool: State<'_, SqlitePool>,
    key: String,
    value: String,
) -> Result<(), String> {
    if !ALLOWED_CONFIG_KEYS.contains(&key.as_str()) {
        return Err(format!("Invalid config key '{}'. Allowed keys: {}", key, ALLOWED_CONFIG_KEYS.join(", ")));
    }

    sqlx::query(
        "INSERT INTO app_config (key, value) VALUES (?, ?)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
    )
    .bind(&key)
    .bind(&value)
    .execute(&*pool)
    .await
    .map(|_| ())
    .map_err(|e| {
        tracing::error!(error = %e, key = %key, "DB error in set_app_config");
        "Failed to save setting".to_string()
    })
}

/// Deletes all user data (jobs, proofs, drafts) and resets the job counter.
/// Companies, people, app_config, and the audit_log are preserved.
/// Requires a confirmation phrase to prevent accidental or malicious invocation.
/// Also cleans up proof files from disk.
#[tauri::command]
pub async fn reset_job_data(
    app: tauri::AppHandle,
    pool: State<'_, SqlitePool>,
    confirmation: String,
) -> Result<(), String> {
    if confirmation != "DELETE ALL JOBS" {
        return Err("Confirmation phrase mismatch. Type exactly: DELETE ALL JOBS".to_string());
    }

    let mut tx = pool.begin().await.map_err(|e| {
        tracing::error!(error = %e, "DB error starting reset transaction");
        "Failed to start reset".to_string()
    })?;

    // Audit the reset BEFORE deleting anything
    let log_id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now();
    sqlx::query(
        "INSERT INTO audit_log (id, job_id, event_type, description, actor, timestamp) VALUES (?, NULL, 'DATA_RESET', 'All job data was reset by user', 'User', ?)"
    )
    .bind(&log_id)
    .bind(&now)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
            tracing::error!(error = %e, "DB error logging reset action");
            "Failed to log reset action".to_string()
        })?;

    // Preserve audit_log — it is immutable by design
    sqlx::query("DELETE FROM proofs").execute(&mut *tx).await.map_err(|e| {
        tracing::error!(error = %e, "DB error deleting proofs during reset");
        "Failed to reset proofs".to_string()
    })?;
    sqlx::query("DELETE FROM job_drafts").execute(&mut *tx).await.map_err(|e| {
        tracing::error!(error = %e, "DB error deleting drafts during reset");
        "Failed to reset drafts".to_string()
    })?;
    sqlx::query("DELETE FROM jobs").execute(&mut *tx).await.map_err(|e| {
        tracing::error!(error = %e, "DB error deleting jobs during reset");
        "Failed to reset jobs".to_string()
    })?;
    sqlx::query("UPDATE job_counter SET last_val = 0 WHERE id = 1").execute(&mut *tx).await.map_err(|e| {
        tracing::error!(error = %e, "DB error resetting counter");
        "Failed to reset counter".to_string()
    })?;

    tx.commit().await.map_err(|e| {
        tracing::error!(error = %e, "DB error committing reset");
        "Failed to commit reset".to_string()
    })?;

    // Clean up proof files from disk (outside transaction — best effort)
    if let Ok(app_dir) = app.path().app_data_dir() {
        let proofs_dir = app_dir.join("proofs");
        if proofs_dir.exists() {
            // Remove all job subdirectories
            if let Ok(entries) = fs::read_dir(&proofs_dir) {
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        let _ = fs::remove_dir_all(entry.path());
                    }
                }
            }
        }
    }

    Ok(())
}
