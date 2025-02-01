CREATE TABLE IF NOT EXISTS thread (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,

    title TEXT,
    content TEXT NOT NULL,
    parent_thread INTEGER,

    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    deleted_at DATETIME,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY(parent_thread) REFERENCES thread(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_thread_user_id ON thread(user_id);
CREATE INDEX IF NOT EXISTS idx_thread_parent_thread ON thread(parent_thread);
CREATE INDEX idx_thread_deleted ON thread(is_deleted);

CREATE TRIGGER IF NOT EXISTS update_thread_timestamp
AFTER UPDATE ON thread
FOR EACH ROW
BEGIN
    UPDATE thread
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;