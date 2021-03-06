CREATE TABLE batch_note (
    id SERIAL NOT NULL PRIMARY KEY,
    batch_id INTEGER NOT NULL,
    target VARCHAR NOT NULL,
    time TIMESTAMP NOT NULL,
    value TEXT NOT NULL,
    CONSTRAINT fk_batch FOREIGN KEY (batch_id) REFERENCES batch(id) ON DELETE CASCADE
);
