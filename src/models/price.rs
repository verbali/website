use crate::helpers::database;
use crate::models::Subscription;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Debug, PartialEq,
)]
#[diesel(belongs_to(Subscription))]
#[diesel(table_name = crate::schema::prices)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Price {
    pub id: i32,
    pub country: String,
    pub price: f32,
    pub subscription_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::prices)]
struct NewPrice<'a> {
    country: &'a str,
    price: f32,
    subscription_id: i32,
}

#[derive(Debug)]
pub struct InsertablePrice {
    pub country: String,
    pub price: f32,
    pub subscription_id: i32,
}

impl Price {
    pub fn create(price: InsertablePrice) -> Result<Price, diesel::result::Error> {
        use crate::schema::prices;

        let connection = &mut database::establish_connection();
        let new_price = NewPrice {
            country: &price.country,
            price: price.price,
            subscription_id: price.subscription_id,
        };

        diesel::insert_into(prices::table)
            .values(&new_price)
            .returning(Price::as_returning())
            .get_result(connection)
    }
}
