CREATE TABLE IF NOT EXISTS likes (
    user_id INTEGER NOT NULL,
    thread_id INTEGER NOT NULL,

    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    deleted_at DATETIME,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY(user_id, thread_id),
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY(thread_id) REFERENCES thread(id) ON DELETE CASCADE
);

CREATE INDEX idx_likes_deleted ON likes(is_deleted);