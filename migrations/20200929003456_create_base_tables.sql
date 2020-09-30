CREATE TABLE beer (
    id SERIAL NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL DEFAULT '',
    style VARCHAR NOT NULL DEFAULT ''
);

CREATE TABLE batch (
    id SERIAL NOT NULL PRIMARY KEY,
    beer_id INTEGER NOT NULL,
    date DATE NOT NULL DEFAULT CURRENT_DATE,
    UNIQUE (beer_id, date)
);

CREATE TABLE batch_measurement (
    batch_id INTEGER NOT NULL,
    name VARCHAR NOT NULL,
    value NUMERIC(6, 3) NOT NULL,
    time TIMESTAMP DEFAULT NULL,
    PRIMARY KEY (batch_id, name)
);

CREATE TABLE batch_ingredient (
    id SERIAL NOT NULL PRIMARY KEY,
    batch_id INTEGER NOT NULL,
    ingredient VARCHAR NOT NULL DEFAULT '',
    amount NUMERIC(5, 2) NOT NULL DEFAULT 0,
    unit VARCHAR NOT NULL DEFAULT '',
    time VARCHAR DEFAULT NULL
);

CREATE TABLE batch_note (
    id SERIAL NOT NULL PRIMARY KEY,
    beer_id INTEGER NOT NULL,
    value TEXT NOT NULL
);
