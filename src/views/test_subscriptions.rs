#[cfg(feature = "server")]
use crate::models::{InsertableSubscription, PublicSubscription, Subscription};
use crate::Route;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use verbali_design_system::components::forms::{Button, Input};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AppSubscription {
    pub id: i32,
    pub title: String,
    pub price: f32,
    pub label: String,
}

#[component]
pub fn TestSubscriptions() -> Element {
    let subscriptions = use_server_future(get_all_subscriptions)?;
    let mut subscription_value = use_signal(HashMap::new);

    rsx! {
        div {
            class: "max-w-4xl lg:mx-auto lg:w-4xl p-4",

            div {
                match subscriptions.value().unwrap() {
                    Ok(items) => {
                        rsx! {
                            for subscription in items.iter() {
                                    div {
                                        class: "border border-slate-700 rounded-md p-4 my-2",
                                        "ID: {subscription.id} | Label: {subscription.label} | Title: {subscription.title} | Price: {subscription.price}"
                                    }
                            }
                        }
                    },
                    Err(_) => {
                        rsx! {
                            div {
                                class: "border border-slate-700 rounded-md p-4 my-2",
                                "Nothing!"
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
                        let price = subscription_value.read().get("price").unwrap().as_value().parse::<f32>().unwrap();
                        let label = match subscription_value.read().get("label") {
                            Some(l) => Some(l.as_value()),
                            None => None
                        };

                        match insert_subscription(title, price, label).await
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
                    label: "Price",
                    name: "price",
                    placeholder: "5",
                    required: true
                }
                Input {
                    class: "mb-8",
                    label: "Label",
                    name: "label",
                    placeholder: "popular"
                }
                div {
                    class: "text-center",
                    Button<Route> {
                        label: "Enregistrer",
                        type: "submit"
                    }
                }
            }

            // form {
            //     class: "mt-16",

            //     oninput: move |event| {
            //         price_value.set(event.data().values());
            //     },

            //     onsubmit: move |_| {
            //         async move {
            //             let country = price_value.read().get("country").unwrap().as_value();
            //             let price: f32 = price_value.read().get("price").unwrap().as_value().parse::<f32>().unwrap();
            //             let subscription_id: i32 = price_value.read().get("subscription_id").unwrap().as_value().parse::<i32>().unwrap();

            //             match insert_price(country, price, subscription_id).await
            //             {
            //                 Ok(response) => {
            //                     document::eval(&format!("console.log('{response}')"));
            //                 },
            //                 Err(err) => {
            //                     document::eval(&format!("console.log('{err}')"));
            //                 }
            //             }
            //         }
            //     },

            //     h1 {
            //         class: "font-bold text-6xl my-8",
            //         "Ajouter un prix"
            //     }

            //     Input {
            //         class: "mb-8",
            //         label: "Country",
            //         name: "country",
            //         placeholder: "france",
            //         required: true
            //     }

            //     Input {
            //         class: "mb-8",
            //         label: "Price",
            //         name: "price",
            //         placeholder: "0",
            //         required: true
            //     }

            //     Input {
            //         class: "mb-8",
            //         label: "Subscription ID",
            //         name: "subscription_id",
            //         placeholder: "1",
            //         required: true
            //     }

            //     div {
            //         class: "text-center",
            //         Button<Route> {
            //             label: "Enregistrer",
            //             type: "submit"
            //         }
            //     }
            // }
        }
    }
}

#[server]
async fn get_all_subscriptions() -> Result<Vec<AppSubscription>, ServerFnError> {
    match Subscription::findAll() {
        Ok(subscriptions) => {
            let json = serde_json::to_string(&subscriptions);
            let subscriptions: Vec<AppSubscription> = if let Ok(s) = json {
                serde_json::from_str(&s).unwrap()
            } else {
                Vec::new()
            };

            Ok(subscriptions)
        }
        Err(err) => Err(err.into()),
    }
}

#[server]
async fn insert_subscription(
    title: String,
    price: f32,
    label: Option<String>,
) -> Result<String, ServerFnError> {
    match Subscription::create(InsertableSubscription {
        title: title,
        price: price,
        label: label,
    }) {
        Ok(subscription) => Ok(serde_json::to_string(&subscription).unwrap()),
        Err(err) => Err(err.into()),
    }
}
