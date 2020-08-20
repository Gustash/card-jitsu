

PRAGMA foreign_keys=off;

BEGIN TRANSACTION;

ALTER TABLE challenges RENAME TO challenges_temp;

CREATE TABLE challenges (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  challenger VARCHAR(255) NOT NULL,
  challenged VARCHAR(255) NOT NULL,
  accepted BOOLEAN NOT NULL DEFAULT FALSE,
  winner VARCHAR(255),
  UNIQUE(challenger, challenged)
);

INSERT INTO challenges(id, challenger, challenged, accepted, winner)
SELECT 
  id,
  challenger,
  CASE
    WHEN user_one = challenger THEN
      user_two
    ELSE
      user_one
  END,
  accepted,
  winner
FROM challenges_temp;

DROP TABLE challenges_temp;

COMMIT;

PRAGMA foreign_keys=on;
