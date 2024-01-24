-- Add up migration script here
CREATE TABLE processes
(
    id         SERIAL PRIMARY KEY,
    process_id VARCHAR UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_processes_process_id ON processes (process_id);

CREATE TABLE clients
(
    id         SERIAL PRIMARY KEY,
    client_id  VARCHAR UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_clients_client_id ON clients (client_id);

CREATE TABLE process_owners
(
    process_id INTEGER NOT NULL REFERENCES processes (id),
    client_id  INTEGER NOT NULL REFERENCES clients (id),
    PRIMARY KEY (process_id, client_id)
);

CREATE TABLE documents
(
    id                  VARCHAR PRIMARY KEY,
    process_id          INTEGER   NOT NULL REFERENCES processes (id),
    created_at          TIMESTAMP NOT NULL,
    model_version       VARCHAR   NOT NULL,
    correlation_message VARCHAR,
    transfer_contract   VARCHAR,
    issued              JSONB,
    issuer_connector    JSONB     NOT NULL,
    content_version     VARCHAR,
    recipient_connector JSONB,
    sender_agent        VARCHAR,
    recipient_agent     JSONB,
    payload             BYTEA,
    payload_type        VARCHAR,
    message_id          VARCHAR
);
