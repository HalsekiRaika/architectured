CREATE TABLE streams(
  id      UUID   NOT NULL,
  version SERIAL NOT NULL CHECK ( version > 0 ) ,

  PRIMARY KEY (id, version)
);

CREATE TABLE events(
  id      SERIAL NOT NULL PRIMARY KEY, -- Auto increment
  journal UUID   NOT NULL,
  version SERIAL NOT NULL CHECK ( version > 0 ),
  event   BYTEA  NOT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),

  FOREIGN KEY (journal, version) REFERENCES streams(id, version) ON DELETE CASCADE
);