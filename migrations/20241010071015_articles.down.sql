-- Add down migration script here
DROP TABLE IF EXISTS articles;
DELETE TABLE IF EXISTS tags;
DELETE TABLE IF EXISTS article_tags;