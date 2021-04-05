CREATE TABLE IF NOT EXISTS gitea_tokens
  ( id TEXT NOT NULL
  , user_id TEXT NOT NULL
  , access_token VARCHAR NOT NULL
  , refresh_token VARCHAR NOT NULL
  , PRIMARY KEY (id)
  );
