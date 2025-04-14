use crate::components::button::Button;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct PricingCardProps {
    title: String,
    price: String,
    features: Vec<String>,
}

#[component]
pub fn PricingCard(props: PricingCardProps) -> Element {
    rsx! {
        div {
            class: "flex-1 border border-gray-300 dark:border-gray-800 rounded-lg p-8 m-4 h-fit",

            h2 {
                class: "font-bold text-xl text-center",
                "{props.title}"
            }

            p {
                class: "font-bold text-4xl text-center my-8",

                if props.price == "0" {
                    "Gratuit"
                } else {
                    "{props.price}"

                    span {
                        class: "text-xl",
                        "/mois"
                    }
                }
            }

            div {
                class: "mb-8",

                ul {
                    for item in props.features.iter() {
                        li { "{item}" }
                    }
                }
            }

            Button {
                label: "Sélectionner",
            }
        }
    }
}
