CREATE TABLE report_list_view (
    aggregate_id  TEXT NOT NULL,
    target        TEXT NOT NULL,
    description   TEXT NOT NULL,
    contact_email TEXT,
    status        TEXT NOT NULL,
    PRIMARY KEY (aggregate_id)
);

DROP TABLE IF EXISTS report_detail_view;