CREATE TABLE batch_ingredient (
    id SERIAL NOT NULL PRIMARY KEY,
    batch_id INTEGER NOT NULL,
    fermentable_id INTEGER NOT NULL,
    unit VARCHAR NOT NULL,
    amount NUMERIC(5, 2) NOT NULL DEFAULT 0,
    time VARCHAR DEFAULT NULL
);
