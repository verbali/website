use crate::components::cards::ItemCard;
use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn Product() -> Element {
    rsx! {
        div {
            class: "max-w-4xl lg:mx-auto lg:w-4xl p-4",

            div {
                class: "my-24 text-center",
                h1 {
                    class: "font-bold text-6xl my-8",
                    {tid!("product")}
                }
                p {
                    class: "my-8 text-xl",
                    {tid!("product.content")}
                }
            }

            ItemCard {
                icon: asset!("/assets/icons/clipboard.svg"),
                title: tid!("product-card-1"),
                content: tid!("product-card-1.content")
            }

            ItemCard {
                icon: asset!("/assets/icons/stats.svg"),
                title: tid!("product-card-2"),
                content: tid!("product-card-2.content")
            }

            ItemCard {
                icon: asset!("/assets/icons/check.svg"),
                title: tid!("product-card-3"),
                content: tid!("product-card-3.content")
            }

            ItemCard {
                icon: asset!("/assets/icons/link.svg"),
                title: tid!("product-card-4"),
                content: tid!("product-card-4.content")
            }
        }
    }
}
