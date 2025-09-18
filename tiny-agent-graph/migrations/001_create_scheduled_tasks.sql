CREATE TABLE IF NOT EXISTS scheduled_tasks (
    task_id TEXT PRIMARY KEY,
    flow_yaml TEXT NOT NULL,
    inputs_json TEXT,
    run_at TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending', -- pending | running | done | failed
    attempts INTEGER DEFAULT 0,
    last_error TEXT,
    locked_until TEXT,
    worker_id TEXT,
    created_at TEXT NOT NULL
);
