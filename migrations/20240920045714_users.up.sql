-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    username VARCHAR NOT NULL DEFAULT '',
    email VARCHAR NOT NULL UNIQUE DEFAULT '',
    password VARCHAR NOT NULL DEFAULT '',
    bio VARCHAR NOT NULL DEFAULT '',
    image VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- updated_atを動作させるためのトリガー

CREATE TRIGGER update_users_modtime
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE PROCEDURE update_timestamp();
