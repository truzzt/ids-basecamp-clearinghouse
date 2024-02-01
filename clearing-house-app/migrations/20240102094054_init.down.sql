-- Add down migration script here
DROP TABLE IF EXISTS documents;
DROP TABLE IF EXISTS process_owners;
DROP INDEX IF EXISTS client_id_idx;
DROP TABLE IF EXISTS clients;
DROP INDEX IF EXISTS process_id_idx;
DROP TABLE IF EXISTS processes;