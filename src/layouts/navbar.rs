use crate::Route;
use dioxus::prelude::*;

const LOGO: Asset = asset!("/assets/logo.svg");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            div {
                class: "mx-auto w-4xl flex flex-row justify-between items-center p-4",
                Link {
                    class: "flex flex-row items-center font-bold text-xl font-fredoka",
                    to: Route::Home {},
                    img {
                        src: LOGO,
                        alt: "Logo",
                        class: "h-8 w-8 mr-2"
                    }
                    "verbali"
                }
                div {
                    class: "flex flex-row items-center",
                    Link {
                        class: "hover:text-gray-700 dark:hover:text-gray-300 px-4 py-2",
                        to: Route::Home {},
                        "Accueil"
                    }
                    Link {
                        class: "hover:text-gray-700 dark:hover:text-gray-300 px-4 py-2",
                        to: Route::Product {},
                        "Produit"
                    }
                    Link {
                        class: "hover:text-gray-700 dark:hover:text-gray-300 px-4 py-2",
                        to: Route::Pricing {},
                        "Tarif"
                    }
                    Link {
                        class: "hover:text-gray-700 dark:hover:text-gray-300 px-4 py-2",
                        to: Route::Contact {},
                        "Contact"
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}

// Link {
//     to: Route::Home {},
//     "Home"
// }
// Link {
//     to: Route::Blog { id: 1 },
//     "Blog"
// }
