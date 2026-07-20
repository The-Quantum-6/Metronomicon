CREATE TABLE course_list_view (
    aggregate_id  TEXT        PRIMARY KEY,
    name          TEXT        NOT NULL,
    code          TEXT        NOT NULL,
    field         TEXT        NOT NULL,
    status        TEXT        NOT NULL
);