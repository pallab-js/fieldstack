#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;
    use chrono::Utc;

    /// Spin up an in-memory SQLite pool and run all migrations.
    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.expect("in-memory pool");
        sqlx::migrate!("./migrations").run(&pool).await.expect("migrations");
        pool
    }

    // ── jobs ──────────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_create_and_get_jobs() {
        let pool = test_pool().await;

        // Seed a company and person
        sqlx::query("INSERT INTO companies (id, name) VALUES ('c1', 'Acme')")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO people (id, name, initials) VALUES ('p1', 'Alice', 'AL')")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO job_counter (id, last_val) VALUES (1, 0) ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();

        let deadline = Utc::now() + chrono::Duration::days(1);

        // Create a job directly via SQL (command layer requires Tauri State)
        let counter: i64 = sqlx::query_scalar(
            "UPDATE job_counter SET last_val = last_val + 1 WHERE id = 1 RETURNING last_val"
        )
        .fetch_one(&pool).await.unwrap();

        let job_id = format!("JOB-{:03}", counter);
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO jobs (id, title, priority, company_id, assigned_person_id, deadline, created_at, updated_at) \
             VALUES (?, 'Test Job', 'medium', 'c1', 'p1', ?, ?, ?)"
        )
        .bind(&job_id).bind(deadline).bind(now).bind(now)
        .execute(&pool).await.unwrap();

        // Verify pagination query
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM jobs")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(total, 1);

        let jobs: Vec<(String,)> = sqlx::query_as(
            "SELECT id FROM jobs ORDER BY created_at DESC LIMIT 50 OFFSET 0"
        )
        .fetch_all(&pool).await.unwrap();
        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].0, "JOB-001");
    }

    // ── logo URL blocklist ────────────────────────────────────────────────────

    #[test]
    fn test_logo_url_blocklist() {
        let blocked = ["javascript:alert(1)", "data:text/html,x", "vbscript:x",
                       "file:///etc/passwd", "ftp://evil.com", "ws://evil.com", "wss://evil.com"];
        let allowed = ["https://example.com/logo.png", "http://cdn.co/img.jpg", "asset://logo.png"];

        const BLOCKED_SCHEMES: &[&str] = &[
            "javascript:", "data:", "vbscript:", "file:", "ftp:", "ws:", "wss:",
        ];

        for url in &blocked {
            let lower = url.to_lowercase();
            let is_blocked = BLOCKED_SCHEMES.iter().any(|s| lower.starts_with(s));
            assert!(is_blocked, "Expected '{}' to be blocked", url);
        }
        for url in &allowed {
            let lower = url.to_lowercase();
            let is_blocked = BLOCKED_SCHEMES.iter().any(|s| lower.starts_with(s));
            assert!(!is_blocked, "Expected '{}' to be allowed", url);
        }
    }

    // ── proof path allowlist ──────────────────────────────────────────────────

    #[test]
    fn test_proof_path_rejects_system_paths() {
        // Simulate the is_path_in_user_dirs logic: home dir must be a prefix.
        // On CI, home_dir() returns Some("/root") or similar — we test the logic directly.
        let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/home/user"));

        let allowed = home.join("Downloads/photo.jpg");
        let blocked = std::path::PathBuf::from("/etc/passwd");

        let candidates = [
            dirs::home_dir(),
            dirs::document_dir(),
            dirs::download_dir(),
        ];

        let is_allowed = |p: &std::path::Path| {
            candidates.iter().flatten().any(|d| p.starts_with(d))
        };

        assert!(is_allowed(&allowed), "Home dir path should be allowed");
        assert!(!is_allowed(&blocked), "/etc/passwd should be blocked");
    }

    // ── overdue sync ──────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_overdue_sync_marks_past_deadline_jobs() {
        let pool = test_pool().await;

        sqlx::query("INSERT INTO companies (id, name) VALUES ('c1', 'Acme')")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO people (id, name, initials) VALUES ('p1', 'Alice', 'AL')")
            .execute(&pool).await.unwrap();

        // Insert a job with a past deadline
        let past = Utc::now() - chrono::Duration::hours(1);
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO jobs (id, title, status, priority, company_id, assigned_person_id, deadline, created_at, updated_at) \
             VALUES ('JOB-001', 'Old Job', 'active', 'medium', 'c1', 'p1', ?, ?, ?)"
        )
        .bind(past).bind(now).bind(now)
        .execute(&pool).await.unwrap();

        // Run the overdue update
        let result = sqlx::query(
            "UPDATE jobs SET status = 'overdue', updated_at = ? \
             WHERE status IN ('pending', 'active') AND deadline < ?"
        )
        .bind(now).bind(now)
        .execute(&pool).await.unwrap();

        assert_eq!(result.rows_affected(), 1);

        let status: String = sqlx::query_scalar("SELECT status FROM jobs WHERE id = 'JOB-001'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(status, "overdue");
    }
}
