CREATE TABLE events (
    id      BLOB PRIMARY KEY,
    time    INTEGER NOT NULL,
    name    TEXT NOT NULL
) STRICT;

CREATE TABLE activities (
    id      BLOB PRIMARY KEY,
    start   INTEGER NOT NULL,
    end     INTEGER,
    name    TEXT NOT NULL
) STRICT;

CREATE INDEX events_time_idx ON events (time);
CREATE INDEX events_name_idx ON events (name);
CREATE INDEX activities_start_idx ON activities (start);
CREATE INDEX activities_end_idx ON activities (end);
CREATE INDEX activities_name_idx ON activities (name);
