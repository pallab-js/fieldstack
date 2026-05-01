use sqlx::SqlitePool;
use tauri::{AppHandle, Emitter};
use tokio::time::{sleep, Duration};
use chrono::Utc;

pub async fn start_overdue_poller(app: AppHandle, pool: SqlitePool) {
    tokio::spawn(async move {
        loop {
            match sync_overdue_statuses(&pool).await {
                Ok(updated_count) => {
                    if updated_count > 0 {
                        let _ = app.emit("overdue-sync", updated_count);
                    }
                }
                Err(e) => eprintln!("Error in overdue poller: {}", e),
            }
            sleep(Duration::from_secs(60)).await;
        }
    });
}

async fn sync_overdue_statuses(pool: &SqlitePool) -> Result<u64, sqlx::Error> {
    let now = Utc::now();

    let result = sqlx::query(
        "UPDATE jobs SET status = 'overdue', updated_at = ? WHERE status IN ('pending', 'active') AND deadline < ?"
    )
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

#[tauri::command]
pub async fn manual_sync_overdue(pool: tauri::State<'_, SqlitePool>) -> Result<u64, String> {
    sync_overdue_statuses(&pool).await.map_err(|e| {
        eprintln!("DB error in manual_sync_overdue: {}", e);
        "Failed to sync overdue jobs".to_string()
    })
}
