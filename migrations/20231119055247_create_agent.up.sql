-- Add up migration script here
CREATE TABLE IF NOT EXISTS tbl_agents (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    mobile VARCHAR(50) NOT NULL,
    photo VARCHAR NOT NULL DEFAULT 'default.png',
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    password VARCHAR(100) NOT NULL,
    assign_to INTEGER,
    agent_code VARCHAR(255),
    state INTEGER,
    city INTEGER,
    status SMALLINT,
    working_area jsonb,
    qualification jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    FOREIGN KEY(assign_to) REFERENCES tbl_users(id)
);

CREATE INDEX agents_email_idx ON tbl_agents (email);