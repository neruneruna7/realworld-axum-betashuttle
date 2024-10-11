-- Add up migration script here

CREATE TABLE IF NOT EXISTS articles (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  body VARCHAR NOT NULL,
  slug VARCHAR NOT NULL UNIQUE,
  author_id UUID NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE
);


-- updated_atを動作させるためのトリガー

CREATE TRIGGER update_articles_modtime
BEFORE UPDATE ON articles
FOR EACH ROW
EXECUTE PROCEDURE update_timestamp();
