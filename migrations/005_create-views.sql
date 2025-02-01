CREATE TABLE IF NOT EXISTS views (
    thread_id INTEGER NOT NULL,
    view_count INTEGER DEFAULT 0,
    PRIMARY KEY(thread_id),
    FOREIGN KEY(thread_id) REFERENCES thread(id) ON DELETE CASCADE
);