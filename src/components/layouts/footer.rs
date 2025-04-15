use crate::components::icons::{FacebookIcon, GithubIcon, InstagramIcon, LinkedinIcon};
use crate::components::logos::FullLogo;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        div {
            class: "max-w-4xl lg:mx-auto lg:w-4xl p-4 mt-32",
            div {
                class: "md:flex md:flex-row md:justify-between md:items-center",

                div {
                    class: "md:flex-1 mb-8",
                    FullLogo {},
                    span {
                        class: "text-sm text-gray-400 dark:text-gray-600",
                        "© 2025, verbali"
                    }
                },

                div {
                    class: "md:flex-1 flex flex-row justify-center",

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
