-- Your SQL goes here
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    name VARCHAR NOT NULL,
    country_id INTEGER
);

CREATE TABLE countries (
    id INTEGER PRIMARY KEY,
    name VARCHAR NOT NULL
);
