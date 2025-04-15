use crate::components::button::Button;
use crate::components::logos::IconLogo;
use crate::Route;
use dioxus::prelude::*;

const LOGO: Asset = asset!("/assets/logo.svg");

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
                        "Récupération de feedback client"
                    }
                    p {
                        class: "my-8 text-xl",
                        "Collectez facilement les avis de vos clients et améliorez votre produit."
                    }
                    Button {
                        label: "En savoir plus",
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
