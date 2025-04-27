use crate::Route;
use dioxus::prelude::*;
use dioxus_i18n::tid;
use verbali_design_system::components::{
    forms::{Button, ButtonLink},
    logos::IconLogo,
};

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "max-w-4xl lg:mx-auto lg:w-4xl p-4",

            div {
                class: "flex flex-col-reverse md:flex-row md:items-center my-24",

                div {
                    class: "md:pr-18",
                    h1 {
                        class: "font-bold text-6xl my-8",
                        {tid!("home-product")}
                    }
                    p {
                        class: "my-8 text-xl",
                        {tid!("home-product.content")}
                    }
                    Button<Route> {
                        label: tid!("home-product.btn"),
                        link: ButtonLink::Internal(Route::Product {})
                    }
                }

                div {
                    class: "p-8",
                    IconLogo {
                        width: "w-64 md:w-240",
                        class: "mx-auto"
                    }
                }
            }
        }
    }
}
