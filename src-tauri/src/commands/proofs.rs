use std::fs;
use std::path::Path;
use sqlx::SqlitePool;
use tauri::{AppHandle, State};
use chrono::Utc;
use uuid::Uuid;
use crate::db::get_proofs_path;

/// Allowed file extensions for proof uploads (prevents executable/script uploads)
const ALLOWED_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "gif", "webp", "bmp",
    "pdf",
    "mp4", "webm", "mov", "avi",
    "mp3", "wav", "m4a", "ogg",
    "doc", "docx", "txt", "csv", "xls", "xlsx",
];

/// Maximum proof file size: 50 MB
const MAX_FILE_SIZE: u64 = 50 * 1024 * 1024;

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

    // Validate file exists
    if !source.exists() {
        return Err("Source file does not exist".to_string());
    }

    // Reject path traversal: source must be a regular file, not a symlink to sensitive system files
    let metadata = source.symlink_metadata().map_err(|e| e.to_string())?;
    if metadata.is_symlink() {
        let canonical = source.canonicalize().map_err(|e| e.to_string())?;
        // Block access to sensitive system directories via symlinks
        let canonical_str = canonical.to_string_lossy().to_lowercase();
        if canonical_str.contains("/etc/") || canonical_str.contains("/proc/") || canonical_str.contains("/sys/") {
            return Err("Access to system files is not allowed".to_string());
        }
    }

    // Validate file extension against allowlist
    let extension = source
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or("File must have a valid extension")?;

    if !ALLOWED_EXTENSIONS.contains(&extension.to_lowercase().as_str()) {
        return Err(format!("File type '.{}' is not allowed. Supported types: {}", extension, ALLOWED_EXTENSIONS.join(", ")));
    }

    // Validate file size
    let file_size = metadata.len();
    if file_size > MAX_FILE_SIZE {
        return Err(format!("File too large ({:.1} MB). Maximum allowed: {} MB", file_size as f64 / 1024.0 / 1024.0, MAX_FILE_SIZE / 1024 / 1024));
    }

    let file_id = Uuid::new_v4().to_string();
    let filename = format!("{}.{}", file_id, extension);

    let proofs_dir = get_proofs_path(&app, &job_id);
    if !proofs_dir.exists() {
        fs::create_dir_all(&proofs_dir).map_err(|e| e.to_string())?;
    }

    let dest_path = proofs_dir.join(&filename);

    // Final defense: ensure dest_path is inside proofs_dir (prevents any dest-side traversal)
    let canonical_dest = dest_path.canonicalize().unwrap_or_else(|_| dest_path.clone());
    let canonical_proofs = proofs_dir.canonicalize().unwrap_or_else(|_| proofs_dir.clone());
    if !canonical_dest.starts_with(&canonical_proofs) {
        return Err("Invalid file destination".to_string());
    }

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
    .map_err(|e| {
        eprintln!("DB error in save_proof_file: {}", e);
        "Failed to save proof record".to_string()
    })?;

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
        .map_err(|e| {
            eprintln!("DB error in proof audit log: {}", e);
            "Failed to write audit log".to_string()
        })?;

    Ok(file_id)
}
