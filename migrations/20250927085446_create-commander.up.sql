-- Add down migration script here
BEGIN;

CREATE TABLE commanders (
	id uuid PRIMARY KEY DEFAULT uuidv7(),
	"name" TEXT,
	nation TEXT,
	age int2,
	military_force int4,
	created_at timestamptz DEFAULT NOW(),
	updated_at timestamptz DEFAULT NOW()
);

CREATE TRIGGER trigger_set_updated_at
BEFORE UPDATE ON commanders
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();

COMMIT;
