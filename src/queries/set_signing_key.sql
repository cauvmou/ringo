BEGIN;
DELETE FROM signing_key WHERE TRUE;
INSERT INTO signing_key (pk_signing_key) VALUES (:key);
COMMIT;