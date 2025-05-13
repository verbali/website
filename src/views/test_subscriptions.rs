#[cfg(feature = "server")]
use crate::models::{
    InsertablePrice, InsertableSubscription, Price, PublicSubscription, Subscription,
};
use crate::Route;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use verbali_design_system::components::forms::{Button, Input};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppPrice {
    pub id: i32,
    pub country: String,
    pub price: f32,
    pub subscription_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppSubscription {
    pub id: i32,
    pub title: String,
    pub highlighted: bool,
    pub prices: Option<Vec<AppPrice>>,
}

#[component]
pub fn TestSubscriptions() -> Element {
    let subscriptions_results = use_server_future(get_all_subscriptions)?;
    let subscriptions: Vec<AppSubscription> = if let Ok(s) = subscriptions_results.value().unwrap()
    {
        serde_json::from_str(&s).unwrap()
    } else {
        Vec::new()
    };

    let mut subscription_value = use_signal(HashMap::new);
    let mut price_value = use_signal(HashMap::new);

    rsx! {
        div {
            class: "max-w-4xl lg:mx-auto lg:w-4xl p-4",

            div {
                h1 {
                    class: "font-bold text-6xl my-8",
                    "Existant"
                }

                for subscription in subscriptions.iter() {
                        div {
                            class: "border border-slate-700 rounded-md p-4 my-2",
                            "ID: {subscription.id} | Highlighted: {subscription.highlighted} | Title: {subscription.title}"

                            if let Some(prices) = &subscription.prices {
                                div {
                                    class: "p-2",
                                    "Prices:"

                                    for price in prices.iter() {
                                        div {
                                            class: "mx-2",
                                            "Country: {price.country} | Price: {price.price}"
                                        }
                                    }
                                }
                            }
                        }
                }
            }

            form {
                class: "mt-16",

                oninput: move |event| {
                    subscription_value.set(event.data().values());
                },

                onsubmit: move |_| {
                    async move {
                        let title = subscription_value.read().get("title").unwrap().as_value();
                        let highlighted = match subscription_value.read().get("highlighted") {
                            Some(value) => value.as_value(),
                            None => "off".to_string(),
                        };

                        match insert_subscription(title, highlighted).await
                        {
                            Ok(response) => {
                                document::eval(&format!("console.log('{response}')"));
                            },
                            Err(err) => {
                                document::eval(&format!("console.log('{err}')"));
                            }
                        }
                    }
                },

                h1 {
                    class: "font-bold text-6xl my-8",
                    "Ajouter un abonnement"
                }

                Input {
                    class: "mb-8",
                    label: "Title",
                    name: "title",
                    placeholder: "Basique",
                    required: true
                }
                Input {
                    class: "mb-8",
                    type: "checkbox",
                    name: "highlighted",
                    label: "Highlighted"
                }
                div {
                    class: "text-center",
                    Button<Route> {
                        label: "Enregistrer",
                        type: "submit"
                    }
                }
            }

            form {
                class: "mt-16",

                oninput: move |event| {
                    price_value.set(event.data().values());
                },

                onsubmit: move |_| {
                    async move {
                        let country = price_value.read().get("country").unwrap().as_value();
                        let price: f32 = price_value.read().get("price").unwrap().as_value().parse::<f32>().unwrap();
                        let subscription_id: i32 = price_value.read().get("subscription_id").unwrap().as_value().parse::<i32>().unwrap();

                        match insert_price(country, price, subscription_id).await
                        {
                            Ok(response) => {
                                document::eval(&format!("console.log('{response}')"));
                            },
                            Err(err) => {
                                document::eval(&format!("console.log('{err}')"));
                            }
                        }
                    }
                },

                h1 {
                    class: "font-bold text-6xl my-8",
                    "Ajouter un prix"
                }

                Input {
                    class: "mb-8",
                    label: "Country",
                    name: "country",
                    placeholder: "france",
                    required: true
                }

                Input {
                    class: "mb-8",
                    label: "Price",
                    name: "price",
                    placeholder: "0",
                    required: true
                }

                Input {
                    class: "mb-8",
                    label: "Subscription ID",
                    name: "subscription_id",
                    placeholder: "1",
                    required: true
                }

                div {
                    class: "text-center",
                    Button<Route> {
                        label: "Enregistrer",
                        type: "submit"
                    }
                }
            }
        }
    }
}

#[server]
async fn get_all_subscriptions() -> Result<String, ServerFnError> {
    match Subscription::findAll() {
        Ok(subscriptions) => {
            let json = serde_json::to_string(&subscriptions).unwrap();
            Ok(json)
        }
        Err(err) => Err(err.into()),
    }
}

#[server]
async fn insert_subscription(title: String, highlighted: String) -> Result<String, ServerFnError> {
    match Subscription::create(InsertableSubscription {
        title: title,
        highlighted: if highlighted == "on" { true } else { false },
    }) {
        Ok(subscription) => Ok(serde_json::to_string(&subscription).unwrap()),
        Err(err) => Err(err.into()),
    }
}

#[server]
async fn insert_price(
    country: String,
    price: f32,
    subscription_id: i32,
) -> Result<String, ServerFnError> {
    match Price::create(InsertablePrice {
        country,
        price,
        subscription_id,
    }) {
        Ok(price) => Ok(serde_json::to_string(&price).unwrap()),
        Err(err) => Err(err.into()),
    }
}
