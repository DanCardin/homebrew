CREATE TABLE batch_fermentable (
    id SERIAL NOT NULL PRIMARY KEY,
    batch_id INTEGER NOT NULL,
    fermentable_id INTEGER,
    amount NUMERIC(5, 2) NOT NULL DEFAULT 0,
    unit VARCHAR DEFAULT NULL,
    time TIME WITHOUT TIME ZONE DEFAULT NULL,
    CONSTRAINT fk_batch FOREIGN KEY (batch_id) REFERENCES batch(id),
    CONSTRAINT fk_fermentable FOREIGN KEY (fermentable_id) REFERENCES fermentable(id)
);
