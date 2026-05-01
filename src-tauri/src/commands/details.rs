use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;
use chrono::{DateTime, Utc};
use crate::commands::jobs::Job;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Proof {
    pub id: String,
    pub job_id: String,
    pub file_path: String,
    pub file_type: String,
    pub submitted_by: String,
    pub submitted_at: DateTime<Utc>,
    pub is_locked: bool,
    pub dispute_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuditLog {
    pub id: String,
    pub job_id: Option<String>,
    pub event_type: String,
    pub description: String,
    pub actor: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobDetails {
    pub job: Job,
    pub proofs: Vec<Proof>,
    pub audit_log: Vec<AuditLog>,
}

fn db_err(msg: &str) -> impl Fn(sqlx::Error) -> String {
    let msg = msg.to_string();
    move |e| {
        tracing::error!(error = %e, operation = %msg, "Database error");
        format!("Failed to {}", msg)
    }
}

#[tauri::command]
pub async fn get_job_details(
    pool: State<'_, SqlitePool>,
    job_id: String,
) -> Result<JobDetails, String> {
    let job = sqlx::query_as::<_, Job>(
        "SELECT * FROM jobs WHERE id = ?"
    )
    .bind(&job_id)
    .fetch_one(&*pool)
    .await
    .map_err(db_err("fetch job details"))?;

    let proofs = sqlx::query_as::<_, Proof>(
        "SELECT * FROM proofs WHERE job_id = ? ORDER BY submitted_at DESC"
    )
    .bind(&job_id)
    .fetch_all(&*pool)
    .await
    .map_err(db_err("fetch proofs"))?;

    let audit_log = sqlx::query_as::<_, AuditLog>(
        "SELECT * FROM audit_log WHERE job_id = ? ORDER BY timestamp DESC"
    )
    .bind(&job_id)
    .fetch_all(&*pool)
    .await
    .map_err(db_err("fetch audit log"))?;

    Ok(JobDetails {
        job,
        proofs,
        audit_log,
    })
}
