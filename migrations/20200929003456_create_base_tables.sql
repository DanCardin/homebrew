CREATE TABLE beer (
    id SERIAL NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL DEFAULT '',
    style VARCHAR NOT NULL DEFAULT ''
);

CREATE TABLE batch (
    id SERIAL NOT NULL PRIMARY KEY,
    beer_id INTEGER NOT NULL,
    date DATE NOT NULL DEFAULT CURRENT_DATE,
    UNIQUE (beer_id, date),
    CONSTRAINT fk_beer FOREIGN KEY (beer_id) REFERENCES beer(id)
);

CREATE TABLE batch_measurement (
    batch_id INTEGER NOT NULL,
    name VARCHAR NOT NULL,
    value NUMERIC(6, 3) NOT NULL,
    time TIMESTAMP DEFAULT NULL,
    PRIMARY KEY (batch_id, name),
    CONSTRAINT fk_batch FOREIGN KEY (batch_id) REFERENCES batch(id)
);

CREATE TABLE batch_note (
    id SERIAL NOT NULL PRIMARY KEY,
    beer_id INTEGER NOT NULL,
    value TEXT NOT NULL,
    CONSTRAINT fk_beer FOREIGN KEY (beer_id) REFERENCES beer(id)
);
