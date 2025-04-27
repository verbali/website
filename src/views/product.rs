use dioxus::prelude::*;
use dioxus_i18n::tid;
use verbali_design_system::components::{
    cards::{ItemCard, ItemCardIcon},
    icons::{CheckIcon, ClipboardIcon, LinkIcon, StatsIcon},
};

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
                icon: ItemCardIcon::Element(rsx!(ClipboardIcon { class: "size-16 mx-auto sm:mx-8", color: "#3399ff" })),
                title: tid!("product-card-1"),
                content: tid!("product-card-1.content")
            }

            ItemCard {
                icon: ItemCardIcon::Element(rsx!(StatsIcon { class: "size-16 mx-auto sm:mx-8", color: "#3399ff" })),
                title: tid!("product-card-2"),
                content: tid!("product-card-2.content")
            }

            ItemCard {
                icon: ItemCardIcon::Element(rsx!(CheckIcon { class: "size-16 mx-auto sm:mx-8", color: "#3399ff" })),
                title: tid!("product-card-3"),
                content: tid!("product-card-3.content")
            }

            ItemCard {
                icon: ItemCardIcon::Element(rsx!(LinkIcon { class: "size-16 mx-auto sm:mx-8", color: "#3399ff" })),
                title: tid!("product-card-4"),
                content: tid!("product-card-4.content")
            }
        }
    }
}
