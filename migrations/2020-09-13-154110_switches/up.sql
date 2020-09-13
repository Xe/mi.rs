PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS members
  ( id INT PRIMARY KEY
  , cmene TEXT UNIQUE NOT NULL
  , picurl TEXT UNIQUE NOT NULL
  );

CREATE TABLE IF NOT EXISTS switches
  ( id TEXT UNIQUE NOT NULL PRIMARY KEY
  , who TEXT NOT NULL
  , started_at TEXT NOT NULL
  , ended_at TEXT
  , duration TEXT NOT NULL
  , FOREIGN KEY (who)
      REFERENCES members(cmene)
  );
