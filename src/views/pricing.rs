use crate::components::cards::PricingCard;
use dioxus::prelude::*;

#[component]
pub fn Pricing() -> Element {
    rsx! {
        div {
            class: "max-w-4xl lg:mx-auto lg:w-4xl p-4",

            div {
                class: "my-24 text-center",
                h1 {
                    class: "font-bold text-6xl my-8",
                    "Tarification"
                }
                p {
                    class: "my-8 text-xl",
                    "Choisissez le plan adapté à vos besoins."
                }
            }

            div {
                class: "flex flex-row flex-wrap justify-center",

                PricingCard {
                    title: "Basique",
                    price: "0",
                    features: vec![
                        "Feature 1".to_string(),
                        "Feature 2".to_string(),
                    ],
                }

                PricingCard {
                    title: "Standard",
                    price: "5€",
                    features: vec![
                        "Feature 1".to_string(),
                        "Feature 2".to_string(),
                        "Feature 3".to_string(),
                        "Feature 4".to_string(),
                    ],
                }

                PricingCard {
                    title: "Premium",
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
