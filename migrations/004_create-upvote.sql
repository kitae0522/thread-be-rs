CREATE TYPE reaction_enum AS ENUM ('UP', 'DOWN');

CREATE TABLE IF NOT EXISTS upvote (
    user_id INTEGER NOT NULL,
    thread_id INTEGER NOT NULL,
    
    reaction reaction_enum NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY(user_id, thread_id),
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY(thread_id) REFERENCES thread(id) ON DELETE CASCADE
);
