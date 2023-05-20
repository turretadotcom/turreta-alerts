CREATE TABLE alerts
(
    id                        SERIAL PRIMARY KEY,
    source                    VARCHAR(50) NOT NULL,
    source_component          VARCHAR(50) NOT NULL,
    alert_type                VARCHAR(50) NOT NULL,
    description               TEXT,
    subject_type              VARCHAR(50) NOT NULL,
    subject_reference_number  VARCHAR(50) NOT NULL,
    subject_description       TEXT,
    content                   TEXT NOT NULL,
    created_at                TIMESTAMP NULL,
    updated_at                TIMESTAMP NULL
);

