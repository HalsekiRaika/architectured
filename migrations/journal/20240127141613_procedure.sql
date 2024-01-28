--
CREATE PROCEDURE vup(stream_id UUID, dest INTEGER, new INTEGER) LANGUAGE plpgsql AS $$
  DECLARE current INTEGER;
BEGIN
  SELECT streams.version INTO current
  FROM streams WHERE id = stream_id;

  IF (NOT FOUND AND dest <> 0) OR (new <> dest) THEN
    RAISE EXCEPTION 'It is possible that a `streams` was not found, or the version number is not what was expected. expect: %, got: %', dest, new;
  END IF;

  INSERT INTO streams(id, version)
    VALUES (stream_id, new)
  ON CONFLICT (id) DO
    UPDATE SET version = new;
END;
$$