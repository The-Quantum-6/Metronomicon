CREATE TYPE user_role AS ENUM ('user', 'admin', 'root');
ALTER TABLE users
ADD COLUMN role user_role NOT NULL DEFAULT 'user';

DROP TABLE IF EXISTS course;

CREATE TABLE IF NOT EXISTS courses (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    content TEXT,
    name TEXT NOT NULL,
    code VARCHAR(10) NOT NULL
);
