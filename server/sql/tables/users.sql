-- TODO Handle timezones properly

CREATE TABLE Users (
    id          SERIAL NOT NULL PRIMARY KEY,
    email       TEXT NOT NULL UNIQUE,
    username    TEXT NOT NULL UNIQUE CHECK (char_length(first_name) < 40),
    password    TEXT NOT NULL CHECK (char_length(first_name) < 40),
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

