CREATE TABLE nodes (
    public_key VARCHAR(66) PRIMARY KEY NOT NULL,
    alias VARCHAR(255) NOT NULL,
    capacity BIGINT NOT NULL,
    first_seen BIGINT NOT NULL
);