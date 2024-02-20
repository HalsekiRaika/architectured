CREATE TABLE streams(
  id UUID NOT NULL PRIMARY KEY
);

CREATE TABLE events(
  id      UUID    NOT NULL DEFAULT gen_random_uuid(), -- Auto Generated
  stream  UUID    NOT NULL,
  version INTEGER NOT NULL CHECK ( version > 0 ),
  event   JSON    NOT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),

  FOREIGN KEY (stream) REFERENCES streams(id) ON DELETE CASCADE,

  PRIMARY KEY (id)
);