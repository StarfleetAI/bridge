-- Add migration script here

ALTER TABLE agents ADD COLUMN execution_steps_limit INTEGER DEFAULT NULL;
