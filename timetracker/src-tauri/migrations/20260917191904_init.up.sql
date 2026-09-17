CREATE TABLE timestamps (
    time    INTEGER PRIMARY KEY
);

CREATE TABLE events (
    id      TEXT PRIMARY KEY,
    time    INTEGER NOT NULL REFERENCES timestamps(time),
    name    TEXT NOT NULL
) STRICT;

CREATE TABLE activities (
    id      TEXT PRIMARY KEY,
    start   INTEGER NOT NULL REFERENCES timestamps(time),
    end     INTEGER NOT NULL REFERENCES timestamps(time),
    name    TEXT NOT NULL
) STRICT;

CREATE INDEX events_name_idx ON events (name);
CREATE INDEX activities_name_idx ON activities (name);
