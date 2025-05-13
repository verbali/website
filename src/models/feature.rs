use crate::helpers::database;
use crate::models::Subscription;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::features)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Feature {
    pub id: i32,
    pub feature: String,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Subscription))]
#[diesel(belongs_to(Feature))]
#[diesel(table_name = crate::schema::subscriptions_features)]
#[diesel(primary_key(subscription_id, feature_id))]
pub struct SubscriptionFeature {
    pub subscription_id: i32,
    pub feature_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::features)]
struct NewFeature<'a> {
    feature: &'a str,
}

#[derive(Debug)]
pub struct InsertableFeature {
    pub feature: String,
}

impl Feature {
    pub fn create(feature: InsertableFeature) -> Result<Feature, diesel::result::Error> {
        use crate::schema::features;

        let connection = &mut database::establish_connection();
        let new_feature = NewFeature {
            feature: &feature.feature,
        };

        diesel::insert_into(features::table)
            .values(&new_feature)
            .returning(Feature::as_returning())
            .get_result(connection)
    }
}
