CREATE TABLE fermentable (
    id SERIAL NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    country VARCHAR,
    category VARCHAR NOT NULL,
    kind VARCHAR NOT NULL,
    color INTEGER NOT NULL,
    ppg DECIMAL NOT NULL
);
