CREATE TABLE report_detail_view (
    aggregate_id  TEXT NOT NULL,
    target        TEXT NOT NULL,
    description   TEXT NOT NULL,
    contact_email TEXT,
    status        TEXT NOT NULL,
    PRIMARY KEY (aggregate_id)
);