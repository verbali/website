use crate::helpers::database;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::subscriptions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Subscription {
    pub id: i32,
    pub title: String,
    pub highlighted: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::subscriptions)]
struct NewSubscription<'a> {
    title: &'a str,
    highlighted: bool,
}

#[derive(Debug)]
pub struct InsertableSubscription {
    pub title: String,
    pub highlighted: bool,
}

impl Subscription {
    pub fn create(
        subscription: InsertableSubscription,
    ) -> Result<Subscription, diesel::result::Error> {
        use crate::schema::subscriptions;

        let connection = &mut database::establish_connection();
        let new_subscription = NewSubscription {
            title: &subscription.title,
            highlighted: subscription.highlighted,
        };

        diesel::insert_into(subscriptions::table)
            .values(&new_subscription)
            .returning(Subscription::as_returning())
            .get_result(connection)
    }

    pub fn findAll() -> Result<Vec<Subscription>, diesel::result::Error> {
        use crate::schema::subscriptions;

        let connection = &mut database::establish_connection();

        subscriptions::dsl::subscriptions
            .select(Subscription::as_select())
            .load(connection)
    }
}
