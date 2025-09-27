-- Add down migration script here
BEGIN;

CREATE TABLE troops (
	id uuid PRIMARY KEY DEFAULT uuidv7(),
	"name" TEXT,
	tribe TEXT,
    "type" TEXT,
	attack_power int4,
    commander_id uuid,
	created_at timestamptz DEFAULT NOW(),
	updated_at timestamptz DEFAULT NOW(),
    CONSTRAINT fk_commander
        FOREIGN KEY(commander_id)
        REFERENCES commanders(id)
        ON DELETE CASCADE
);

CREATE TRIGGER trigger_set_updated_at
BEFORE UPDATE ON troops
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();

COMMIT;

