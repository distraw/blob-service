-- Add migration script here
CREATE USER signer;

CREATE TABLE notes (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL
);
