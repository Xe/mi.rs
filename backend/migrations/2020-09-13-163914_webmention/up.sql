CREATE TABLE IF NOT EXISTS webmentions
  ( id TEXT UNIQUE NOT NULL PRIMARY KEY
  , source_url TEXT NOT NULL
  , target_url TEXT NOT NULL
  );
