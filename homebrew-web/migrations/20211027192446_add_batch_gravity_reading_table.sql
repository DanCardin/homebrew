CREATE TABLE batch_gravity_reading (
    id SERIAL NOT NULL PRIMARY KEY,
    batch_id INTEGER NOT NULL,
    time TIMESTAMP NOT NULL,
    value NUMERIC(6, 3) NOT NULL,
    CONSTRAINT fk_batch FOREIGN KEY (batch_id) REFERENCES batch(id) ON DELETE CASCADE
);
