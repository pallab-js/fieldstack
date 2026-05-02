-- Fieldstack initial schema

CREATE TABLE IF NOT EXISTS companies (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    logo_url TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS levels (
    id TEXT PRIMARY KEY,
    company_id TEXT NOT NULL,
    name TEXT NOT NULL,
    parent_id TEXT,
    sort_order INTEGER DEFAULT 0,
    FOREIGN KEY (company_id) REFERENCES companies(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES levels(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS people (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    avatar_color TEXT,
    initials TEXT,
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS person_companies (
    person_id TEXT NOT NULL,
    company_id TEXT NOT NULL,
    PRIMARY KEY (person_id, company_id),
    FOREIGN KEY (person_id) REFERENCES people(id) ON DELETE CASCADE,
    FOREIGN KEY (company_id) REFERENCES companies(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS jobs (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    priority TEXT NOT NULL DEFAULT 'medium',
    company_id TEXT NOT NULL,
    assigned_person_id TEXT NOT NULL,
    deadline DATETIME NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    completion_date DATETIME,
    FOREIGN KEY (company_id) REFERENCES companies(id) ON DELETE CASCADE,
    FOREIGN KEY (assigned_person_id) REFERENCES people(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS proofs (
    id TEXT PRIMARY KEY,
    job_id TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_type TEXT NOT NULL,
    submitted_by TEXT NOT NULL,
    submitted_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    is_locked INTEGER DEFAULT 1,
    dispute_reason TEXT,
    FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE,
    FOREIGN KEY (submitted_by) REFERENCES people(id)
);

CREATE TABLE IF NOT EXISTS audit_log (
    id TEXT PRIMARY KEY,
    job_id TEXT,
    event_type TEXT NOT NULL,
    description TEXT NOT NULL,
    actor TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    metadata TEXT
);

CREATE TABLE IF NOT EXISTS job_drafts (
    id TEXT PRIMARY KEY,
    payload TEXT NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS app_config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS job_counter (
    last_val INTEGER DEFAULT 0
);

INSERT OR IGNORE INTO job_counter (last_val) VALUES (0);

CREATE INDEX IF NOT EXISTS idx_jobs_status_deadline ON jobs(status, deadline);
CREATE INDEX IF NOT EXISTS idx_jobs_company ON jobs(company_id);
CREATE INDEX IF NOT EXISTS idx_jobs_person ON jobs(assigned_person_id);
CREATE INDEX IF NOT EXISTS idx_jobs_status ON jobs(status);
CREATE INDEX IF NOT EXISTS idx_proofs_job ON proofs(job_id);
CREATE INDEX IF NOT EXISTS idx_audit_job ON audit_log(job_id);
CREATE INDEX IF NOT EXISTS idx_proofs_submitted_by ON proofs(submitted_by);
