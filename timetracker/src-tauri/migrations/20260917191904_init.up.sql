CREATE TABLE events (
    id      BLOB PRIMARY KEY,
    time    INTEGER NOT NULL,
    name    TEXT NOT NULL
) STRICT;

CREATE TABLE activities (
    id      BLOB PRIMARY KEY,
    start   INTEGER NOT NULL,
    end     INTEGER NOT NULL,
    name    TEXT NOT NULL
) STRICT;

CREATE INDEX events_name_idx ON events (name);
CREATE INDEX activities_name_idx ON activities (name);
