use sqlx::SqlitePool;
use tauri::State;
use chrono::Utc;

fn db_err(msg: &str) -> impl Fn(sqlx::Error) -> String {
    let msg = msg.to_string();
    move |e| {
        eprintln!("DB error in {}: {}", msg, e);
        format!("Failed to {}", msg)
    }
}

#[tauri::command]
pub async fn get_draft(pool: State<'_, SqlitePool>, id: String) -> Result<Option<String>, String> {
    let payload: Option<String> = sqlx::query_scalar("SELECT payload FROM job_drafts WHERE id = ?")
        .bind(&id)
        .fetch_optional(&*pool)
        .await
        .map_err(db_err("fetch draft"))?;

    Ok(payload)
}

#[tauri::command]
pub async fn save_draft(pool: State<'_, SqlitePool>, id: String, payload: String) -> Result<(), String> {
    let now = Utc::now();
    sqlx::query("INSERT INTO job_drafts (id, payload, updated_at) VALUES (?, ?, ?) ON CONFLICT(id) DO UPDATE SET payload = ?, updated_at = ?")
        .bind(&id)
        .bind(&payload)
        .bind(now)
        .bind(&payload)
        .bind(now)
        .execute(&*pool)
        .await
        .map_err(db_err("save draft"))?;

    Ok(())
}

#[tauri::command]
pub async fn delete_draft(pool: State<'_, SqlitePool>, id: String) -> Result<(), String> {
    sqlx::query("DELETE FROM job_drafts WHERE id = ?")
        .bind(&id)
        .execute(&*pool)
        .await
        .map_err(db_err("delete draft"))?;

    Ok(())
}
