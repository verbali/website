CREATE TABLE prices (
    id SERIAL PRIMARY KEY,
    country VARCHAR NOT NULL,
    price REAL NOT NULL,
    subscription_id INTEGER NOT NULL REFERENCES subscriptions (id)
)
