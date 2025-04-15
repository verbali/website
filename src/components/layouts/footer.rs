use crate::components::icons::{FacebookIcon, GithubIcon, InstagramIcon, LinkedinIcon};
use crate::components::logos::FullLogo;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        div {
            class: "mx-auto mt-32 w-4xl",
            div {
                class: "flex flex-row justify-between items-center",

                div {
                    class: "flex-1",
                    FullLogo {},
                    "© 2025, verbali"
                },

                div {
                    class: "flex-1 flex flex-row justify-center",

                    div {
                        class: "flex-1",
                        span {
                            class: "font-bold",
                            "Verbali"
                        },
                        ul {
                            li {
                                class: "my-2",
                                Link {
                                    class: "hover:text-gray-700 dark:hover:text-gray-300",
                                    to: Route::Home {},
                                    "Accueil"
                                }
                            },
                            li {
                                class: "my-2",
                                Link {
                                    class: "hover:text-gray-700 dark:hover:text-gray-300",
                                    to: Route::Product {},
                                    "Produit"
                                }
                            },
                            li {
                                class: "my-2",
                                Link {
                                    class: "hover:text-gray-700 dark:hover:text-gray-300",
                                    to: Route::Pricing {},
                                    "Tarif"
                                }
                            },
                            li {
                                class: "my-2",
                                Link {
                                    class: "hover:text-gray-700 dark:hover:text-gray-300",
                                    to: Route::Contact {},
                                    "Contact"
                                }
                            },
                        }
                    },

                    div {
                        class: "flex-1",
                        span {
                            class: "font-bold",
                            "Assistance"
                        },
                        ul {
                            li {
                                class: "my-2",
                                Link {
                                    class: "hover:text-gray-700 dark:hover:text-gray-300",
                                    to: "#",
                                    "Foire aux questions"
                                }
                            },
                            li {
                                class: "my-2",
                                Link {
                                    class: "hover:text-gray-700 dark:hover:text-gray-300",
                                    to: "#",
                                    "Tutoriels"
                                }
                            },
                            li {
                                class: "my-2",
                                Link {
                                    class: "hover:text-gray-700 dark:hover:text-gray-300",
                                    to: "#",
                                    "Demande de support"
                                }
                            },
                        }
                    }
                }
            },
            div {
                class: "flex justify-center items-center mt-8",
                div {
                    class: "m-4",
                    FacebookIcon {}
                },
                div {
                    class: "m-4",
                    LinkedinIcon {}
                },
                div {
                    class: "m-4",
                    InstagramIcon {}
                },
                div {
                    class: "m-4",
                    GithubIcon {}
                },
            }
        }
    }
}
