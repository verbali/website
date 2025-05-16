CREATE TABLE subscriptions (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL UNIQUE,
    price REAL NOT NULL,
    label VARCHAR
)
