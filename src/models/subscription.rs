use crate::helpers::database;
use crate::models::Price;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicSubscription {
    pub id: i32,
    pub title: String,
    pub highlighted: bool,
    pub prices: Option<Vec<Price>>,
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

    pub fn findAll() -> Result<Vec<PublicSubscription>, diesel::result::Error> {
        use crate::schema::subscriptions;

        let connection = &mut database::establish_connection();

        match subscriptions::dsl::subscriptions
            .select(Subscription::as_select())
            .get_results(connection)
        {
            Ok(subscriptions) => {
                let mut results = Vec::new();

                for subscription in subscriptions.iter() {
                    match Price::belonging_to(&subscription)
                        .select(Price::as_select())
                        .get_results(connection)
                    {
                        Ok(prices) => {
                            results.push(PublicSubscription {
                                id: subscription.id,
                                title: subscription.title.clone(),
                                highlighted: subscription.highlighted,
                                prices: Some(prices),
                            });
                        }
                        Err(_) => {
                            results.push(PublicSubscription {
                                id: subscription.id,
                                title: subscription.title.clone(),
                                highlighted: subscription.highlighted,
                                prices: None,
                            });
                        }
                    }
                }

                Ok(results)
            }
            Err(err) => Err(err),
        }
    }
}
