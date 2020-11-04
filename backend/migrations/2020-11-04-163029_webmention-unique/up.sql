CREATE UNIQUE INDEX webmentions_source_target
  ON webmentions(source_url, target_url);
