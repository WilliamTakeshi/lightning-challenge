CREATE TABLE nodes (
    public_key VARCHAR(66) PRIMARY KEY,
    alias VARCHAR(255),
    capacity NUMERIC(18, 8),
    first_seen TIMESTAMPTZ
);