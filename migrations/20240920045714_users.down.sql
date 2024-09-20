-- Add down migration script here
DROP TABLE users;
DROP FUNCTION update_timestamp;
DROP TRIGGER update_users_modtime;