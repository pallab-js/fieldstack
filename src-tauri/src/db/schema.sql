-- Fieldstack SQLite Schema

-- Enable WAL mode for better concurrency (handled in Rust init)
-- PRAGMA journal_mode=WAL;
-- PRAGMA foreign_keys=ON;

-- Companies
CREATE TABLE IF NOT EXISTS companies (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    logo_url TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Organization Levels (e.g., Division, Region, Team)
CREATE TABLE IF NOT EXISTS levels (
    id TEXT PRIMARY KEY,
    company_id TEXT NOT NULL,
    name TEXT NOT NULL,
    parent_id TEXT,
    sort_order INTEGER DEFAULT 0,
    FOREIGN KEY (company_id) REFERENCES companies(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES levels(id) ON DELETE SET NULL
);

-- People
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

-- Junction table for People and Companies (Multiple companies per person)
CREATE TABLE IF NOT EXISTS person_companies (
    person_id TEXT NOT NULL,
    company_id TEXT NOT NULL,
    PRIMARY KEY (person_id, company_id),
    FOREIGN KEY (person_id) REFERENCES people(id) ON DELETE CASCADE,
    FOREIGN KEY (company_id) REFERENCES companies(id) ON DELETE CASCADE
);

-- Jobs
CREATE TABLE IF NOT EXISTS jobs (
    id TEXT PRIMARY KEY, -- JOB-001 format
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'pending', -- pending, active, overdue, disputed, resolved, completed
    priority TEXT NOT NULL DEFAULT 'medium', -- low, medium, high, critical
    company_id TEXT NOT NULL,
    assigned_person_id TEXT NOT NULL,
    deadline DATETIME NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    completion_date DATETIME,
    FOREIGN KEY (company_id) REFERENCES companies(id),
    FOREIGN KEY (assigned_person_id) REFERENCES people(id)
);

-- Proofs
CREATE TABLE IF NOT EXISTS proofs (
    id TEXT PRIMARY KEY,
    job_id TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_type TEXT NOT NULL, -- photo, video, audio, document
    submitted_by TEXT NOT NULL, -- person_id
    submitted_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    is_locked INTEGER DEFAULT 1,
    dispute_reason TEXT,
    FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE,
    FOREIGN KEY (submitted_by) REFERENCES people(id)
);

-- Audit Log (Immutable)
CREATE TABLE IF NOT EXISTS audit_log (
    id TEXT PRIMARY KEY,
    job_id TEXT,
    event_type TEXT NOT NULL, -- CREATE, STATUS_CHANGE, PROOF_ADD, DISPUTE, RESOLVE
    description TEXT NOT NULL,
    actor TEXT NOT NULL, -- name of the user
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    metadata TEXT -- JSON encoded metadata
);

-- Job Drafts (For Wizard persistence)
CREATE TABLE IF NOT EXISTS job_drafts (
    id TEXT PRIMARY KEY, -- draft_id or person_id
    payload TEXT NOT NULL, -- JSON blob of the 5-step wizard state
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- App Configuration
CREATE TABLE IF NOT EXISTS app_config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- Job Counter (For generating JOB-001 IDs)
CREATE TABLE IF NOT EXISTS job_counter (
    last_val INTEGER DEFAULT 0
);

-- Insert initial counter if not exists
INSERT OR IGNORE INTO job_counter (last_val) VALUES (0);
