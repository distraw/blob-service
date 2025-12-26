CREATE USER signer WITH PASSWORD 'signer_password_0';

CREATE TABLE IF NOT EXISTS notes (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL
);