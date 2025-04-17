use crate::components::button::Button;
use crate::components::logos::IconLogo;
use crate::Route;
use dioxus::prelude::*;
use dioxus_i18n::tid;

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
                    Button {
                        label: tid!("home-product.btn"),
                        route: Route::Product {}
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
