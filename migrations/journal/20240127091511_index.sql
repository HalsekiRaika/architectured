CREATE INDEX stream_id_index            ON streams(id);
CREATE INDEX event_id_with_stream_index ON events(id, stream, version);