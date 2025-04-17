use crate::components::cards::PricingCard;
use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn Pricing() -> Element {
    rsx! {
        div {
            class: "max-w-4xl lg:mx-auto lg:w-4xl p-4",

            div {
                class: "my-24 text-center",
                h1 {
                    class: "font-bold text-6xl my-8",
                    {tid!("pricing")}
                }
                p {
                    class: "my-8 text-xl",
                    {tid!("pricing.content")}
                }
            }

            div {
                class: "flex flex-row flex-wrap justify-center",

                PricingCard {
                    title: tid!("subscription.one"),
                    price: "0",
                    features: vec![
                        "Feature 1".to_string(),
                        "Feature 2".to_string(),
                    ],
                }

                PricingCard {
                    title: tid!("subscription.two"),
                    price: "5€",
                    features: vec![
                        "Feature 1".to_string(),
                        "Feature 2".to_string(),
                        "Feature 3".to_string(),
                        "Feature 4".to_string(),
                    ],
                }

                PricingCard {
                    title: tid!("subscription.three"),
                    price: "20€",
                    features: vec![
                        "Feature 1".to_string(),
                        "Feature 2".to_string(),
                        "Feature 3".to_string(),
                        "Feature 4".to_string(),
                        "Feature 5".to_string(),
                    ],
                }
            }
        }
    }
}
