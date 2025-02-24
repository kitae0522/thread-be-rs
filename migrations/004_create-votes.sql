CREATE TYPE reaction_enum AS ENUM ('UP', 'DOWN');

CREATE TABLE IF NOT EXISTS votes (
    user_id BIGINT NOT NULL,
    thread_id BIGINT NOT NULL,
    
    reaction reaction_enum NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY(user_id, thread_id),
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY(thread_id) REFERENCES thread(id) ON DELETE CASCADE
);
