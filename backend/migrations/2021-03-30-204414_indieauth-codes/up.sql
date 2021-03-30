CREATE TABLE IF NOT EXISTS indieauth_codes
  ( code TEXT UNIQUE NOT NULL PRIMARY KEY
  , client_id TEXT NOT NULL
  , redirect_uri TEXT NOT NULL
  , "state" TEXT NOT NULL
  , response_type TEXT NOT NULL
  );
