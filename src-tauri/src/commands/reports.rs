use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportSummary {
    pub total_jobs: i64,
    pub completed: i64,
    pub active: i64,
    pub overdue: i64,
    pub disputed: i64,
    pub resolved: i64,
    pub pending: i64,
    pub completion_rate: f64,
    pub avg_completion_days: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobsByCompany {
    pub company_id: String,
    pub company_name: String,
    pub total: i64,
    pub completed: i64,
    pub overdue: i64,
    pub active: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobsByPerson {
    pub person_id: String,
    pub person_name: String,
    pub initials: String,
    pub avatar_color: Option<String>,
    pub total: i64,
    pub completed: i64,
    pub overdue: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusOverTime {
    pub month: String,
    pub completed: i64,
    pub overdue: i64,
    pub created: i64,
}

fn db_err(msg: &str) -> impl Fn(sqlx::Error) -> String {
    let msg = msg.to_string();
    move |e| {
        eprintln!("DB error in {}: {}", msg, e);
        format!("Failed to {}", msg)
    }
}

#[tauri::command]
pub async fn get_report_summary(pool: State<'_, SqlitePool>) -> Result<ReportSummary, String> {
    let total_jobs: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM jobs")
        .fetch_one(&*pool).await.map_err(db_err("count total jobs"))?;

    let completed: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM jobs WHERE status = 'completed'")
        .fetch_one(&*pool).await.map_err(db_err("count completed jobs"))?;

    let active: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM jobs WHERE status = 'active'")
        .fetch_one(&*pool).await.map_err(db_err("count active jobs"))?;

    let overdue: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM jobs WHERE status = 'overdue'")
        .fetch_one(&*pool).await.map_err(db_err("count overdue jobs"))?;

    let disputed: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM jobs WHERE status = 'disputed'")
        .fetch_one(&*pool).await.map_err(db_err("count disputed jobs"))?;

    let resolved: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM jobs WHERE status = 'resolved'")
        .fetch_one(&*pool).await.map_err(db_err("count resolved jobs"))?;

    let pending: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM jobs WHERE status = 'pending'")
        .fetch_one(&*pool).await.map_err(db_err("count pending jobs"))?;

    let completion_rate = if total_jobs > 0 {
        (completed as f64 / total_jobs as f64) * 100.0
    } else {
        0.0
    };

    let avg_completion_days: f64 = sqlx::query_scalar(
        "SELECT COALESCE(AVG((julianday(completion_date) - julianday(created_at))), 0.0) FROM jobs WHERE status = 'completed' AND completion_date IS NOT NULL"
    )
    .fetch_one(&*pool).await.map_err(db_err("calculate avg completion days"))?;

    Ok(ReportSummary {
        total_jobs,
        completed,
        active,
        overdue,
        disputed,
        resolved,
        pending,
        completion_rate,
        avg_completion_days,
    })
}

#[tauri::command]
pub async fn get_jobs_by_company(pool: State<'_, SqlitePool>) -> Result<Vec<JobsByCompany>, String> {
    let rows = sqlx::query_as::<_, (String, String, i64, i64, i64, i64)>(
        r#"
        SELECT
            c.id,
            c.name,
            COUNT(j.id) as total,
            SUM(CASE WHEN j.status = 'completed' THEN 1 ELSE 0 END) as completed,
            SUM(CASE WHEN j.status = 'overdue' THEN 1 ELSE 0 END) as overdue,
            SUM(CASE WHEN j.status = 'active' THEN 1 ELSE 0 END) as active
        FROM companies c
        LEFT JOIN jobs j ON j.company_id = c.id
        GROUP BY c.id, c.name
        ORDER BY total DESC
        "#
    )
    .fetch_all(&*pool)
    .await
    .map_err(db_err("fetch company report"))?;

    Ok(rows.into_iter().map(|(company_id, company_name, total, completed, overdue, active)| {
        JobsByCompany { company_id, company_name, total, completed, overdue, active }
    }).collect())
}

#[tauri::command]
pub async fn get_jobs_by_person(pool: State<'_, SqlitePool>) -> Result<Vec<JobsByPerson>, String> {
    let rows = sqlx::query_as::<_, (String, String, String, Option<String>, i64, i64, i64)>(
        r#"
        SELECT
            p.id,
            p.name,
            p.initials,
            p.avatar_color,
            COUNT(j.id) as total,
            SUM(CASE WHEN j.status = 'completed' THEN 1 ELSE 0 END) as completed,
            SUM(CASE WHEN j.status = 'overdue' THEN 1 ELSE 0 END) as overdue
        FROM people p
        LEFT JOIN jobs j ON j.assigned_person_id = p.id
        GROUP BY p.id, p.name, p.initials, p.avatar_color
        ORDER BY total DESC
        "#
    )
    .fetch_all(&*pool)
    .await
    .map_err(db_err("fetch person report"))?;

    Ok(rows.into_iter().map(|(person_id, person_name, initials, avatar_color, total, completed, overdue)| {
        JobsByPerson { person_id, person_name, initials, avatar_color, total, completed, overdue }
    }).collect())
}

#[tauri::command]
pub async fn get_jobs_by_status_over_time(pool: State<'_, SqlitePool>) -> Result<Vec<StatusOverTime>, String> {
    let rows = sqlx::query_as::<_, (String, i64, i64, i64)>(
        r#"
        SELECT
            strftime('%Y-%m', created_at) as month,
            SUM(CASE WHEN status = 'completed' THEN 1 ELSE 0 END) as completed,
            SUM(CASE WHEN status = 'overdue' THEN 1 ELSE 0 END) as overdue,
            COUNT(*) as created
        FROM jobs
        WHERE created_at >= date('now', '-6 months')
        GROUP BY month
        ORDER BY month ASC
        "#
    )
    .fetch_all(&*pool)
    .await
    .map_err(db_err("fetch status over time report"))?;

    Ok(rows.into_iter().map(|(month, completed, overdue, created)| {
        StatusOverTime { month, completed, overdue, created }
    }).collect())
}
