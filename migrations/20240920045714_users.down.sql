-- Add down migration script here
DROP TRIGGER IF EXISTS update_users_modtime ON users;
-- DROP FUNCTION IF EXISTS update_timestamp;
DROP TABLE IF EXISTS users;
