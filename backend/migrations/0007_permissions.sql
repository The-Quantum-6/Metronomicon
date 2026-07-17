CREATE TABLE IF NOT EXISTS permissions (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    resource_id TEXT NOT NULL,
    perms INTEGER NOT NULL DEFAULT 2 (perms >= 0) 
    UNIQUE (user_id, resource_id)
);

CREATE TABLE IF NOT EXISTS groups (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    name TEXT NOT NULL,
);

CREATE TABLE IF NOT EXISTS group_course(
    group_id UUID NOT NULL,
    course_id UUID NOT NULL,
    PRIMARY KEY (group_id, course_id)
);