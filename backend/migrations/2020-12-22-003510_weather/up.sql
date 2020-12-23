CREATE TABLE IF NOT EXISTS weather
  ( ts TIMESTAMP NOT NULL PRIMARY KEY
  , region TEXT NOT NULL
  , body BLOB NOT NULL -- JSON-encoded weather data
  );

CREATE UNIQUE INDEX weather_ts_region
  ON weather(ts, region);
