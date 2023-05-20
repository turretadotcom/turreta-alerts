CREATE TABLE alerts
(
    id                        BIGINT PRIMARY KEY,
    source                    VARCHAR(50) NOT NULL,
    source_component          VARCHAR(50) NOT NULL,
    type                      VARCHAR(50) NOT NULL,
    description               TEXT,
    subject_type              VARCHAR(50) NOT NULL,
    subject_reference_number  VARCHAR(50) NOT NULL,
    subject_description       VARCHAR(50) NULL,
    content                   TEXT,
    created_at                TIMESTAMP,
    updated_at                TIMESTAMP
);

