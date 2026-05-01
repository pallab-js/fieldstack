use sqlx::SqlitePool;
use tauri::{AppHandle, Emitter};
use tokio::time::{sleep, Duration};
use chrono::Utc;
use uuid::Uuid;

pub async fn start_overdue_poller(app: AppHandle, pool: SqlitePool) {
    tokio::spawn(async move {
        loop {
            match sync_overdue_statuses(&pool).await {
                Ok(updated_count) => {
                    if updated_count > 0 {
                        tracing::info!(count = updated_count, "Overdue jobs synced");
                        let _ = app.emit("overdue-sync", updated_count);
                    }
                }
                Err(e) => {
                    tracing::error!(error = %e, "Error in overdue poller");
                    // Sleep briefly on error to avoid tight loop
                    sleep(Duration::from_secs(10)).await;
                    continue;
                }
            }
            sleep(Duration::from_secs(60)).await;
        }
    });
}

async fn sync_overdue_statuses(pool: &SqlitePool) -> Result<u64, String> {
    let now = Utc::now();

    // First, get the IDs of jobs that will be marked overdue (for audit logging)
    let overdue_job_ids: Vec<String> = sqlx::query_scalar(
        "SELECT id FROM jobs WHERE status IN ('pending', 'active') AND deadline < ?"
    )
    .bind(now)
    .fetch_all(pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "DB error fetching overdue jobs");
        "Failed to fetch overdue jobs".to_string()
    })?;

    if overdue_job_ids.is_empty() {
        return Ok(0);
    }

    // Update the statuses
    let result = sqlx::query(
        "UPDATE jobs SET status = 'overdue', updated_at = ? WHERE status IN ('pending', 'active') AND deadline < ?"
    )
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "DB error updating overdue statuses");
        "Failed to update overdue statuses".to_string()
    })?;

    let updated_count = result.rows_affected();

    // Write audit log entries for each overdue job
    let mut tx = pool.begin().await.map_err(|e| {
        tracing::error!(error = %e, "DB error starting overdue audit tx");
        "Failed to start audit transaction".to_string()
    })?;

    for job_id in &overdue_job_ids {
        let log_id = Uuid::new_v4().to_string();
        let _ = sqlx::query(
            "INSERT INTO audit_log (id, job_id, event_type, description, actor, timestamp) VALUES (?, ?, 'OVERDUE', 'Job automatically marked as overdue by system', 'System', ?)"
        )
        .bind(&log_id)
        .bind(job_id)
        .bind(&now)
        .execute(&mut *tx)
        .await;
    }

    let _ = tx.commit().await;

    Ok(updated_count)
}

#[tauri::command]
pub async fn manual_sync_overdue(pool: tauri::State<'_, SqlitePool>) -> Result<u64, String> {
    tracing::info!("Manual overdue sync requested");
    sync_overdue_statuses(&pool).await
}
