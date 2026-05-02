use std::fs;
use std::path::Path;
use sqlx::SqlitePool;
use tauri::{AppHandle, State};
use chrono::Utc;
use uuid::Uuid;
use crate::db::get_proofs_path;

/// Returns true if `path` is within any standard user-accessible directory.
/// Uses the `dirs` crate for OS-agnostic home/document/download paths.
fn is_path_in_user_dirs(path: &std::path::Path) -> bool {
    let candidates = [
        dirs::home_dir(),
        dirs::document_dir(),
        dirs::download_dir(),
        dirs::desktop_dir(),
        dirs::picture_dir(),
        dirs::video_dir(),
        dirs::audio_dir(),
    ];
    candidates.into_iter().flatten().any(|d| path.starts_with(d))
}

/// Allowed file extensions for proof uploads (prevents executable/script uploads)
const ALLOWED_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "gif", "webp", "bmp",
    "pdf",
    "mp4", "webm", "mov", "avi",
    "mp3", "wav", "m4a", "ogg",
    "doc", "docx", "txt", "csv", "xls", "xlsx",
];

/// Allowed proof type values
const ALLOWED_PROOF_TYPES: &[&str] = &["photo", "video", "audio", "document"];

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

    // Validate proof_type against allowlist
    if !ALLOWED_PROOF_TYPES.contains(&proof_type.as_str()) {
        return Err(format!("Invalid proof_type '{}'. Allowed: {}", proof_type, ALLOWED_PROOF_TYPES.join(", ")));
    }

    // Validate file exists
    if !source.exists() {
        return Err("Source file does not exist".to_string());
    }

    // OS-agnostic path safety: canonicalize and verify the path is within user-accessible directories.
    // This blocks access to system files on all platforms (macOS, Windows, Linux).
    let canonical = source.canonicalize().map_err(|_| "Cannot resolve file path".to_string())?;
    if !is_path_in_user_dirs(&canonical) {
        return Err("Access to system files is not allowed".to_string());
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
    let file_size = canonical.metadata().map_err(|_| "Cannot read file metadata".to_string())?.len();
    if file_size > MAX_FILE_SIZE {
        return Err(format!("File too large ({:.1} MB). Maximum allowed: {} MB", file_size as f64 / 1024.0 / 1024.0, MAX_FILE_SIZE / 1024 / 1024));
    }

    // Validate submitted_by is a valid person
    let person_exists: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM people WHERE id = ?")
        .bind(&submitted_by)
        .fetch_one(&*pool)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "DB error validating submitted_by");
            "Failed to validate submitter".to_string()
        })?;

    if person_exists == 0 {
        return Err(format!("Invalid submitted_by: person '{}' not found", submitted_by));
    }

    let file_id = Uuid::new_v4().to_string();
    let filename = format!("{}.{}", file_id, extension);

    let proofs_dir = get_proofs_path(&app, &job_id);
    if !proofs_dir.exists() {
        fs::create_dir_all(&proofs_dir).map_err(|_| "Failed to create proof directory".to_string())?;
    }

    let dest_path = proofs_dir.join(&filename);

    // Final defense: ensure dest_path is inside proofs_dir (prevents any dest-side traversal)
    // Use parent directory for canonicalize check since dest doesn't exist yet
    let canonical_proofs = proofs_dir.canonicalize().unwrap_or_else(|_| proofs_dir.clone());
    if !dest_path.starts_with(&canonical_proofs) {
        return Err("Invalid file destination".to_string());
    }

    fs::copy(source, &dest_path).map_err(|_| "Failed to copy file".to_string())?;

    let now = Utc::now();
    let dest_path_str = dest_path.to_str().ok_or("Failed to convert path to string")?.to_string();

    // Wrap DB operations — if they fail, clean up the copied file
    let result = (|| async {
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
            tracing::error!(error = %e, "DB error in save_proof_file");
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
                tracing::error!(error = %e, "DB error in proof audit log");
                "Failed to write audit log".to_string()
            })?;

        Ok(file_id)
    })().await;

    // If DB operations failed, clean up the file
    if result.is_err() {
        let _ = fs::remove_file(&dest_path);
    }

    result
}
