use sqlx::SqlitePool;
use tauri::{AppHandle, Emitter};
use tokio::sync::Notify;
use tokio::time::{sleep, Duration};
use chrono::Utc;
use uuid::Uuid;
use std::sync::Arc;

pub async fn start_overdue_poller(app: AppHandle, pool: SqlitePool, notify: Arc<Notify>) {
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
                    sleep(Duration::from_secs(10)).await;
                    continue;
                }
            }

            // Compute how long to sleep: until the next job deadline or 60s, whichever is sooner.
            let sleep_secs = next_deadline_secs(&pool).await.unwrap_or(60).clamp(1, 60);

            tokio::select! {
                _ = sleep(Duration::from_secs(sleep_secs)) => {}
                _ = notify.notified() => {
                    tracing::debug!("Overdue poller woken by new job");
                }
            }
        }
    });
}

/// Returns seconds until the nearest pending/active job deadline, or None if no such jobs exist.
async fn next_deadline_secs(pool: &SqlitePool) -> Option<u64> {
    let next: Option<chrono::DateTime<Utc>> = sqlx::query_scalar(
        "SELECT MIN(deadline) FROM jobs WHERE status IN ('pending', 'active') AND deadline > ?"
    )
    .bind(Utc::now())
    .fetch_one(pool)
    .await
    .ok()
    .flatten();

    next.map(|dt| {
        let secs = (dt - Utc::now()).num_seconds();
        secs.max(0) as u64
    })
}

async fn sync_overdue_statuses(pool: &SqlitePool) -> Result<u64, String> {
    let now = Utc::now();

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
