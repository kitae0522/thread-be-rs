CREATE TABLE IF NOT EXISTS thread (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    
    title TEXT,
    content TEXT NOT NULL,
    parent_thread BIGINT,
    
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    deleted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY(parent_thread) REFERENCES thread(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_thread_user_id ON thread(user_id);
CREATE INDEX IF NOT EXISTS idx_thread_parent_thread ON thread(parent_thread);
CREATE INDEX IF NOT EXISTS idx_thread_deleted ON thread(is_deleted);