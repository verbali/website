// @generated automatically by Diesel CLI.

diesel::table! {
    features (id) {
        id -> Int4,
        feature -> Varchar,
    }
}

diesel::table! {
    subscriptions (id) {
        id -> Int4,
        title -> Varchar,
        price -> Float4,
        label -> Nullable<Varchar>,
    }
}

diesel::table! {
    subscriptions_features (subscription_id, feature_id) {
        subscription_id -> Int4,
        feature_id -> Int4,
    }
}

diesel::joinable!(subscriptions_features -> features (feature_id));
diesel::joinable!(subscriptions_features -> subscriptions (subscription_id));

diesel::allow_tables_to_appear_in_same_query!(features, subscriptions, subscriptions_features,);
