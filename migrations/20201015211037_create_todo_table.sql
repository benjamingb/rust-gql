-- Add migration script here
CREATE TABLE todo (
    id SERIAL PRIMARY KEY,
    body TEXT NULL,
    complete TEXT NULL
);