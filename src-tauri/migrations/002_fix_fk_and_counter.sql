-- Fix #25: assigned_person_id FK used ON DELETE SET NULL on a NOT NULL column — contradiction.
-- Change to ON DELETE RESTRICT so deleting a person with active jobs is blocked.
-- SQLite does not support ALTER TABLE DROP CONSTRAINT, so we recreate the table.

PRAGMA foreign_keys = OFF;

CREATE TABLE jobs_new (
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
    FOREIGN KEY (assigned_person_id) REFERENCES people(id) ON DELETE RESTRICT
);

INSERT INTO jobs_new SELECT * FROM jobs;
DROP TABLE jobs;
ALTER TABLE jobs_new RENAME TO jobs;

CREATE INDEX IF NOT EXISTS idx_jobs_status_deadline ON jobs(status, deadline);
CREATE INDEX IF NOT EXISTS idx_jobs_company ON jobs(company_id);
CREATE INDEX IF NOT EXISTS idx_jobs_person ON jobs(assigned_person_id);
CREATE INDEX IF NOT EXISTS idx_jobs_status ON jobs(status);

PRAGMA foreign_keys = ON;

-- Fix #26: job_counter had no uniqueness constraint — add a single-row enforcement.
-- Recreate with a CHECK constraint so only one row (id=1) can ever exist.

CREATE TABLE job_counter_new (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    last_val INTEGER NOT NULL DEFAULT 0
);

INSERT INTO job_counter_new (id, last_val)
    SELECT 1, COALESCE(MAX(last_val), 0) FROM job_counter;

DROP TABLE job_counter;
ALTER TABLE job_counter_new RENAME TO job_counter;
