-- Add up migration script here
BEGIN;

-- Drop the trigger if it exists
DROP TRIGGER IF EXISTS trigger_set_updated_at ON commanders;

-- Drop the table
DROP TABLE IF EXISTS commanders;

COMMIT;