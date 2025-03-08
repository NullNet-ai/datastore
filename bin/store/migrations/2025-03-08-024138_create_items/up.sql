-- Your SQL goes here
CREATE TABLE items
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR NOT NULL,
    description TEXT
);
