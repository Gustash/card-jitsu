PRAGMA foreign_keys=off;

BEGIN TRANSACTION;

ALTER TABLE challenges RENAME TO challenges_temp;

CREATE TABLE challenges (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  user_one VARCHAR(255) NOT NULL,
  user_two VARCHAR(255) NOT NULL,
  challenger VARCHAR(255) NOT NULL,
  accepted BOOLEAN NOT NULL DEFAULT FALSE,
  winner VARCHAR(255)
);

CREATE UNIQUE INDEX UniqueChallenge ON challenges (user_one, user_two);

INSERT INTO challenges(id, user_one, user_two, challenger, accepted, winner)
SELECT 
  id,
  CASE
    WHEN CAST(challenger AS INTEGER) < CAST(challenged AS INTEGER) THEN
      challenger
    ELSE
      challenged
  END,
  CASE
    WHEN CAST(challenger AS INTEGER) > CAST(challenged AS INTEGER) THEN
      challenger
    ELSE
      challenged
  END,
  challenger,
  accepted,
  winner
FROM challenges_temp;

DROP TABLE challenges_temp;

COMMIT;

PRAGMA foreign_keys=on;
