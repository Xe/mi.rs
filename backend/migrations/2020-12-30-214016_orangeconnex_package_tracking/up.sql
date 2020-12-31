CREATE TABLE IF NOT EXISTS orangeconnex_packages
  ( tracking_number TEXT UNIQUE NOT NULL PRIMARY KEY
  , recieved BOOLEAN NOT NULL DEFAULT false
  );

CREATE TABLE IF NOT EXISTS orangeconnex_traces
  ( id TEXT UNIQUE NOT NULL PRIMARY KEY
  , tracking_number TEXT NOT NULL
  , description TEXT NOT NULL
  , city TEXT
  , country TEXT NOT NULL
  , time_recorded TEXT NOT NULL
  , time_zone TEXT NOT NULL
  , ts INTEGER NOT NULL
  , FOREIGN KEY (tracking_number)
      REFERENCES orangeconnex_packages(tracking_number)
  );

CREATE UNIQUE INDEX orangeconnex_traces_time
  ON orangeconnex_traces(time_recorded, time_zone, ts);
