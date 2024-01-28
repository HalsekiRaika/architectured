CREATE TABLE streams(
  id      UUID   NOT NULL,
  version SERIAL NOT NULL CHECK ( version > 0 ) ,

  PRIMARY KEY (id, version)
);

CREATE TABLE events(
  id      INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- Auto increment
  stream  UUID    NOT NULL,
  version SERIAL  NOT NULL CHECK ( version > 0 ),
  event   JSON    NOT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),

  FOREIGN KEY (stream, version) REFERENCES streams(id, version) ON DELETE CASCADE
);