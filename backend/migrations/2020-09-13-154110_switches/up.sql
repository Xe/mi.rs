PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS members
  ( id INTEGER UNIQUE NOT NULL PRIMARY KEY
  , cmene TEXT UNIQUE NOT NULL
  , picurl TEXT UNIQUE NOT NULL
  );

CREATE TABLE IF NOT EXISTS switches
  ( id TEXT UNIQUE NOT NULL PRIMARY KEY
  , member_id INTEGER NOT NULL
  , started_at TIMESTAMP NOT NULL
  , ended_at TIMESTAMP
  , duration INTEGER
  , FOREIGN KEY (member_id)
      REFERENCES members(id)
  );
