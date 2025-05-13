CREATE TABLE subscriptions_features (
    subscription_id INTEGER REFERENCES subscriptions (id),
    feature_id INTEGER REFERENCES features (id),
    PRIMARY KEY (subscription_id, feature_id)
);
