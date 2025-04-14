use crate::components::button::Button;
use crate::Route;
use dioxus::prelude::*;

const LOGO: Asset = asset!("/assets/logo.svg");

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "mx-auto w-4xl",

            div {
                class: "flex flex-row items-center my-24",

                div {
                    class: "pr-18",
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
                    img {
                        src: LOGO,
                        alt: "Logo",
                        class: "w-240"
                    }
                }
            }

            div {
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus ornare ex vel lectus euismod congue. Nunc vitae ante sed metus posuere convallis. Praesent aliquam mauris ut erat pulvinar pulvinar. Praesent elit erat, ornare in aliquam quis, convallis sit amet tellus. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Morbi est neque, vulputate vel est sed, viverra auctor tortor. Aenean ultrices interdum pulvinar. Proin ut sapien a risus aliquam ullamcorper sed in nisi. Donec nibh justo, cursus eget orci sed, ornare porta dui."
            }
        }
    }
}
