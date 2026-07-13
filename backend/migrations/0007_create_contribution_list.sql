CREATE TABLE contribution_list_view (
    aggregate_id  TEXT        PRIMARY KEY,
    course_id     TEXT        NOT NULL,
    contribution  JSONB       NOT NULL,
    status        TEXT        NOT NULL
);