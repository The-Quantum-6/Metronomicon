CREATE TABLE IF NOT EXISTS permissions (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    group_id TEXT NOT NULL REFERENCES course_query(view_id) ON DELETE CASCADE,
    perms INTEGER NOT NULL DEFAULT 2 (perms >= 0) 
    UNIQUE (user_id, group_id)
);