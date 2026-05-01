mod db;
mod commands;
mod overdue;

use tauri::{Emitter, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            
            // Initialize structured logging with file output
            let log_path = db::init_logging(&app_handle);
            tracing::info!(version = env!("CARGO_PKG_VERSION"), "Fieldstack starting");

            // Check if previous session crashed
            if let Some(crash_context) = db::check_previous_crash(&app_handle) {
                tracing::warn!(%crash_context, "Previous session ended abnormally");
            }

            // Set crash marker for this session
            db::set_crash_marker(&app_handle, "app startup");
            
            // Initialize database asynchronously
            tauri::async_runtime::spawn(async move {
                match db::init_db(&app_handle).await {
                    Ok(pool) => {
                        app_handle.manage(pool.clone());

                        // Seed initial data on first launch (no-op if already seeded)
                        if let Err(e) = commands::org::seed_data_inner(&pool).await {
                            tracing::error!(error = %e, "Seed data error");
                        }
                        
                        // Start the background overdue engine
                        overdue::start_overdue_poller(app_handle.clone(), pool).await;
                        
                        // Clear crash marker — startup succeeded
                        db::clear_crash_marker(&app_handle);

                        // Signal readiness to frontend
                        let _ = app_handle.emit("app-ready", true);
                        tracing::info!("Database and Overdue Engine initialized successfully");
                    }
                    Err(e) => {
                        tracing::error!(error = %e, log_file = %log_path.display(), "Failed to initialize database");
                        // Notify frontend of critical failure so it can show error UI
                        let _ = app_handle.emit("db-init-error", "Failed to initialize the local database. Please restart the app and contact support if the issue persists.".to_string());
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_jobs,
            commands::create_job,
            commands::update_job_status,
            commands::dispute_job,
            commands::resolve_job,
            commands::save_proof_file,
            commands::get_draft,
            commands::save_draft,
            commands::delete_draft,
            commands::get_job_details,
            commands::get_companies,
            commands::create_company,
            commands::update_company,
            commands::delete_company,
            commands::get_people,
            commands::create_person,
            commands::update_person,
            commands::delete_person,
            commands::get_person_companies,
            // NOTE: seed_data intentionally NOT exposed — runs only once during setup()
            commands::get_report_summary,
            commands::get_jobs_by_company,
            commands::get_jobs_by_person,
            commands::get_jobs_by_status_over_time,
            overdue::manual_sync_overdue,
            commands::get_app_config,
            commands::set_app_config,
            commands::reset_job_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
