INSERT INTO
  members(id, cmene, picurl)
VALUES
  (0, 'Cadey', 'https://mi.within.website/static/img/cadey.png'),
  (1, 'Nicole', 'https://mi.within.website/static/img/nicole.png'),
  (2, 'Jessie', 'https://mi.within.website/static/img/jessie.png'),
  (3, 'Ashe', 'https://mi.within.website/static/img/ashe.png'),
  (4, 'Sephie', 'https://mi.within.website/static/img/sephie.png'),
  (5, 'Mai', 'https://mi.within.website/static/img/mai.png');

-- testing values
INSERT INTO
  switches(id, member_id, started_at, ended_at, duration)
VALUES
  ( 'ropjar', 0, '2013-10-07 08:23:19.120', datetime('now'), 200),
  ( 'flopnax', 5, datetime('now'), NULL, 0)
