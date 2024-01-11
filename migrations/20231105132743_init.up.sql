-- Add up migration script here
CREATE TABLE IF NOT EXISTS tbl_roles(
    role_id SMALLINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    role_name VARCHAR(255)
);

CREATE TABLE tbl_users (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    mobile VARCHAR(100),
    photo VARCHAR NOT NULL DEFAULT 'default.png',
    verified BOOLEAN NOT NULL DEFAULT TRUE,
    password VARCHAR(100) NOT NULL,
    role SMALLINT,
    address jsonb,
    area_list jsonb,
    qualification jsonb,
    employee_id VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    FOREIGN KEY(role) REFERENCES tbl_roles(role_id)
);

CREATE INDEX users_email_idx ON tbl_users (email);