use crate::components::logos::FullLogo;
use crate::Route;
use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            div {
                class: "max-w-4xl lg:mx-auto lg:w-4xl flex flex-row justify-between items-center p-4",
                Link {
                    to: Route::Home {},
                    FullLogo {}
                }
                div {
                    class: "flex flex-row items-center",
                    Link {
                        class: "hover:text-gray-700 dark:hover:text-gray-300 px-4 py-2",
                        to: Route::Home {},
                        {tid!("menu.home")}
                    }
                    Link {
                        class: "hover:text-gray-700 dark:hover:text-gray-300 px-4 py-2",
                        to: Route::Product {},
                        {tid!("menu.product")}
                    }
                    Link {
                        class: "hover:text-gray-700 dark:hover:text-gray-300 px-4 py-2",
                        to: Route::Pricing {},
                        {tid!("menu.pricing")}
                    }
                    Link {
                        class: "hover:text-gray-700 dark:hover:text-gray-300 px-4 py-2",
                        to: Route::Contact {},
                        {tid!("menu.contact")}
                    }
                }
            }
        }
    }
}
