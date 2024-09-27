-- Add up migration script here
CREATE TABLE IF NOT EXISTS user_follows (
    id UUID PRIMARY KEY,
    follower_id UUID NOT NULL,
    following_id UUID NOT NULL,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (follower_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (following_id) REFERENCES users(id) ON DELETE CASCADE
);