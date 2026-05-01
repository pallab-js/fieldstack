use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Job {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub company_id: String,
    pub assigned_person_id: String,
    pub deadline: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completion_date: Option<DateTime<Utc>>,
}

/// Allowed priority values
const ALLOWED_PRIORITIES: &[&str] = &["low", "medium", "high", "critical"];

/// Allowed status values
const ALLOWED_STATUSES: &[&str] = &["pending", "active", "overdue", "disputed", "resolved", "completed"];

fn db_err(msg: &str) -> impl Fn(sqlx::Error) -> String {
    let msg = msg.to_string();
    move |e| {
        tracing::error!(error = %e, operation = %msg, "Database error");
        format!("Failed to {}", msg)
    }
}

#[tauri::command]
pub async fn get_jobs(
    pool: State<'_, SqlitePool>,
    status_filter: Option<String>,
) -> Result<Vec<Job>, String> {
    // Validate status_filter against allowlist if provided
    if let Some(ref status) = status_filter {
        if !ALLOWED_STATUSES.contains(&status.as_str()) {
            return Err(format!("Invalid status filter '{}'. Allowed: {}", status, ALLOWED_STATUSES.join(", ")));
        }
    }

    let jobs = if let Some(ref status) = status_filter {
        sqlx::query_as::<_, Job>(
            r#"
            SELECT id, title, description, status, priority, company_id, assigned_person_id, 
                   deadline, created_at, updated_at, completion_date
            FROM jobs
            WHERE status = ?
            ORDER BY created_at DESC
            "#
        )
        .bind(status)
        .fetch_all(&*pool)
        .await
    } else {
        sqlx::query_as::<_, Job>(
            r#"
            SELECT id, title, description, status, priority, company_id, assigned_person_id, 
                   deadline, created_at, updated_at, completion_date
            FROM jobs
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&*pool)
        .await
    }
    .map_err(db_err("fetch jobs"))?;

    Ok(jobs)
}

#[tauri::command]
pub async fn create_job(
    pool: State<'_, SqlitePool>,
    title: String,
    description: Option<String>,
    priority: String,
    company_id: String,
    assigned_person_id: String,
    deadline: DateTime<Utc>,
) -> Result<String, String> {
    // Validate priority against allowlist
    if !ALLOWED_PRIORITIES.contains(&priority.as_str()) {
        return Err(format!("Invalid priority '{}'. Allowed: {}", priority, ALLOWED_PRIORITIES.join(", ")));
    }

    // Validate title is not empty
    if title.trim().is_empty() {
        return Err("Job title cannot be empty".to_string());
    }

    // Validate deadline is not in the past
    if deadline < Utc::now() {
        return Err("Deadline cannot be in the past".to_string());
    }

    let mut transaction = pool.begin().await.map_err(db_err("start transaction"))?;

    // Validate referential integrity: company exists
    let company_exists: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM companies WHERE id = ?")
        .bind(&company_id)
        .fetch_one(&mut *transaction)
        .await
        .map_err(db_err("validate company"))?;
    if company_exists == 0 {
        return Err("Company not found".to_string());
    }

    // Validate referential integrity: assigned person exists
    let person_exists: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM people WHERE id = ?")
        .bind(&assigned_person_id)
        .fetch_one(&mut *transaction)
        .await
        .map_err(db_err("validate person"))?;
    if person_exists == 0 {
        return Err("Assigned person not found".to_string());
    }

    let counter: i64 = sqlx::query_scalar("UPDATE job_counter SET last_val = last_val + 1 RETURNING last_val")
        .fetch_one(&mut *transaction)
        .await
        .map_err(db_err("generate job ID"))?;

    let job_id = format!("JOB-{:03}", counter);
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO jobs (id, title, description, priority, company_id, assigned_person_id, deadline, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&job_id)
    .bind(&title)
    .bind(&description)
    .bind(&priority)
    .bind(&company_id)
    .bind(&assigned_person_id)
    .bind(deadline)
    .bind(now)
    .bind(now)
    .execute(&mut *transaction)
    .await
    .map_err(db_err("create job"))?;

    let log_id = uuid::Uuid::new_v4().to_string();
    let log_desc = format!("Job created: {}", title);
    sqlx::query("INSERT INTO audit_log (id, job_id, event_type, description, actor, timestamp) VALUES (?, ?, 'CREATE', ?, 'System', ?)")
        .bind(&log_id)
        .bind(&job_id)
        .bind(&log_desc)
        .bind(&now)
        .execute(&mut *transaction)
        .await
        .map_err(db_err("write audit log"))?;

    transaction.commit().await.map_err(db_err("commit transaction"))?;

    Ok(job_id)
}

#[tauri::command]
pub async fn update_job_status(
    pool: State<'_, SqlitePool>,
    job_id: String,
    status: String,
) -> Result<(), String> {
    // Validate status against allowlist
    if !ALLOWED_STATUSES.contains(&status.as_str()) {
        return Err(format!("Invalid status '{}'. Allowed: {}", status, ALLOWED_STATUSES.join(", ")));
    }

    let now = Utc::now();
    let completion_date = if status == "completed" { Some(now) } else { None };

    // Make status update and audit log atomic
    let mut transaction = pool.begin().await.map_err(db_err("start transaction"))?;

    sqlx::query("UPDATE jobs SET status = ?, updated_at = ?, completion_date = ? WHERE id = ?")
        .bind(&status)
        .bind(now)
        .bind(completion_date)
        .bind(&job_id)
        .execute(&mut *transaction)
        .await
        .map_err(db_err("update job status"))?;

    let log_id = uuid::Uuid::new_v4().to_string();
    let log_desc = format!("Status changed to: {}", status);
    sqlx::query("INSERT INTO audit_log (id, job_id, event_type, description, actor, timestamp) VALUES (?, ?, 'STATUS_CHANGE', ?, 'System', ?)")
        .bind(&log_id)
        .bind(&job_id)
        .bind(&log_desc)
        .bind(&now)
        .execute(&mut *transaction)
        .await
        .map_err(db_err("write audit log"))?;

    transaction.commit().await.map_err(db_err("commit transaction"))?;

    Ok(())
}

#[tauri::command]
pub async fn dispute_job(
    pool: State<'_, SqlitePool>,
    job_id: String,
    reason: String,
) -> Result<(), String> {
    let now = Utc::now();

    let mut transaction = pool.begin().await.map_err(db_err("start transaction"))?;

    sqlx::query("UPDATE jobs SET status = 'disputed', updated_at = ? WHERE id = ?")
        .bind(now).bind(&job_id)
        .execute(&mut *transaction).await.map_err(db_err("dispute job"))?;

    // Only update proofs that don't already have a dispute reason (preserve existing disputes)
    sqlx::query("UPDATE proofs SET dispute_reason = ? WHERE job_id = ? AND (dispute_reason IS NULL OR dispute_reason = '')")
        .bind(&reason).bind(&job_id)
        .execute(&mut *transaction).await.map_err(db_err("update proof dispute reason"))?;

    let log_id = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO audit_log (id, job_id, event_type, description, actor, timestamp) VALUES (?, ?, 'DISPUTE', ?, 'System', ?)")
        .bind(&log_id).bind(&job_id).bind(format!("Disputed: {}", reason)).bind(&now)
        .execute(&mut *transaction).await.map_err(db_err("write audit log"))?;

    transaction.commit().await.map_err(db_err("commit transaction"))?;

    Ok(())
}

#[tauri::command]
pub async fn resolve_job(
    pool: State<'_, SqlitePool>,
    job_id: String,
) -> Result<(), String> {
    let now = Utc::now();

    let mut transaction = pool.begin().await.map_err(db_err("start transaction"))?;

    sqlx::query("UPDATE jobs SET status = 'resolved', updated_at = ? WHERE id = ?")
        .bind(now).bind(&job_id)
        .execute(&mut *transaction).await.map_err(db_err("resolve job"))?;

    let log_id = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO audit_log (id, job_id, event_type, description, actor, timestamp) VALUES (?, ?, 'RESOLVE', 'Dispute resolved by owner', 'System', ?)")
        .bind(&log_id).bind(&job_id).bind(&now)
        .execute(&mut *transaction).await.map_err(db_err("write audit log"))?;

    transaction.commit().await.map_err(db_err("commit transaction"))?;

    Ok(())
}
