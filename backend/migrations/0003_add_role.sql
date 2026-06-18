ALTER table users
ADD COLUMN role NOT NULL DEFAULT 'user'
CREATE TYPE user_role AS ENUM ('user', 'admin', 'root');

DROP table IF EXISTS course;

CREATE TABLE IF NOT EXISTS courses (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    content TEXT,
    name TEXT NOT NULL,
    code VARCHAR(10) NOT NULL
);
