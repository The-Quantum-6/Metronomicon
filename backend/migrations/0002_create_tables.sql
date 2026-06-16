CREATE TABLE IF NOT EXISTS subject (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    content TEXT,
    name TEXT NOT NULL,
    code VARCHAR(10) NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    sub UUID NOT NULL,
    name TEXT NOT NULL,
    email TEXT
);

DROP TABLE IF EXISTS test;