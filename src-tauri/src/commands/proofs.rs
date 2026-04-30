use std::fs;
use std::path::Path;
use sqlx::SqlitePool;
use tauri::{AppHandle, State};
use chrono::Utc;
use uuid::Uuid;
use crate::db::get_proofs_path;

#[tauri::command]
pub async fn save_proof_file(
    app: AppHandle,
    pool: State<'_, SqlitePool>,
    job_id: String,
    proof_type: String,
    source_path: String,
    submitted_by: String,
) -> Result<String, String> {
    let source = Path::new(&source_path);
    if !source.exists() {
        return Err("Source file does not exist".to_string());
    }

    let extension = source.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    
    let file_id = Uuid::new_v4().to_string();
    let filename = format!("{}.{}", file_id, extension);
    
    let proofs_dir = get_proofs_path(&app, &job_id);
    if !proofs_dir.exists() {
        fs::create_dir_all(&proofs_dir).map_err(|e| e.to_string())?;
    }

    let dest_path = proofs_dir.join(filename);
    fs::copy(source, &dest_path).map_err(|e| e.to_string())?;

    let now = Utc::now();
    let dest_path_str = dest_path.to_str().ok_or("Failed to convert path to string")?.to_string();

    sqlx::query(
        "INSERT INTO proofs (id, job_id, file_path, file_type, submitted_by, submitted_at, is_locked) VALUES (?, ?, ?, ?, ?, ?, 1)"
    )
    .bind(&file_id)
    .bind(&job_id)
    .bind(&dest_path_str)
    .bind(&proof_type)
    .bind(&submitted_by)
    .bind(now)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    // Log the addition
    let log_id = Uuid::new_v4().to_string();
    let log_desc = format!("Proof added ({}): {}", proof_type, file_id);
    sqlx::query("INSERT INTO audit_log (id, job_id, event_type, description, actor, timestamp) VALUES (?, ?, 'PROOF_ADD', ?, ?, ?)")
        .bind(&log_id)
        .bind(&job_id)
        .bind(&log_desc)
        .bind(&submitted_by)
        .bind(&now)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(file_id)
}
