use crate::helpers::database;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug, PartialEq)]
#[diesel(table_name = crate::schema::subscriptions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Subscription {
    pub id: i32,
    pub title: String,
    pub price: f32,
    pub label: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::subscriptions)]
struct NewSubscription<'a> {
    title: &'a str,
    price: f32,
    label: Option<&'a str>,
}

#[derive(Debug)]
pub struct InsertableSubscription {
    pub title: String,
    pub price: f32,
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicSubscription {
    pub id: i32,
    pub title: String,
    pub price: f32,
    pub label: Option<String>,
}

impl Subscription {
    pub fn create(
        subscription: InsertableSubscription,
    ) -> Result<Subscription, diesel::result::Error> {
        use crate::schema::subscriptions;

        let connection = &mut database::establish_connection();
        let label = subscription.label.unwrap_or("".to_string());
        let new_subscription = NewSubscription {
            title: &subscription.title,
            price: subscription.price,
            label: Some(&label),
        };

        diesel::insert_into(subscriptions::table)
            .values(&new_subscription)
            .returning(Subscription::as_returning())
            .get_result(connection)
    }

    pub fn findAll() -> Result<Vec<PublicSubscription>, diesel::result::Error> {
        use crate::schema::subscriptions;

        let connection = &mut database::establish_connection();

        match subscriptions::dsl::subscriptions
            .select(Subscription::as_select())
            .order(subscriptions::id.asc())
            .get_results(connection)
        {
            Ok(subscriptions) => {
                let mut results = Vec::new();

                for subscription in subscriptions.iter() {
                    results.push(PublicSubscription {
                        id: subscription.id,
                        title: subscription.title.clone(),
                        price: subscription.price,
                        label: subscription.label.clone(),
                    });
                }

                Ok(results)
            }
            Err(err) => Err(err),
        }
    }
}
