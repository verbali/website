use crate::components::itemcard::ItemCard;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Product() -> Element {
    rsx! {
        div {
            class: "mx-auto w-4xl",

            div {
                class: "my-24 text-center",
                h1 {
                    class: "font-bold text-6xl my-8",
                    "Collectez les avis de vos clients"
                }
                p {
                    class: "my-8 text-xl",
                    "Collectez, Filtrez, et Analysez les avis de vos clients pour améliorer votre produit."
                }
            }

            ItemCard {
                icon: asset!("/assets/icons/clipboard.svg"),
                title: "Enquête personnalisée",
                content: "Configurez votre propre formulaire de feedback avec des questions personalisées."
            }

            ItemCard {
                icon: asset!("/assets/icons/stats.svg"),
                title: "Analyse des réponses",
                content: "Visualisez, consultez et analysez les données pour mieux comprendre vos clients."
            }

            ItemCard {
                icon: asset!("/assets/icons/check.svg"),
                title: "Suivi de la satisfaction",
                content: "Mesurez la satisfaction de vos clients en temps réel grâce au dashboard détaillé.."
            }

            ItemCard {
                icon: asset!("/assets/icons/link.svg"),
                title: "Insitation à la publication",
                content: "Proposez à vos clients satisfait de partager leur expérience sur les plateformes."
            }
        }
    }
}
