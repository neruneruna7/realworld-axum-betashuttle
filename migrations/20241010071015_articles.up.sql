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


-- タグテーブル
CREATE TABLE IF NOT EXISTS tags (
  id SERIAL PRIMARY KEY,
  tag VARCHAR(255) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- タグを記事をつなぐ連関エンティティ
CREATE TABLE IF NOT EXISTS article_tags (
  article_id SERIAL NOT NULL,
  tag_id SERIAL NOT NULL,
  PRIMARY KEY (article_id, tag_id),
  FOREIGN KEY (article_id) REFERENCES articles(id) ON DELETE CASCADE,
  FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);