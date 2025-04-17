use crate::components::icons::{FacebookIcon, GithubIcon, InstagramIcon, LinkedinIcon};
use crate::components::logos::FullLogo;
use crate::Route;
use dioxus::prelude::*;
use dioxus_i18n::{prelude::*, tid};
use unic_langid::LanguageIdentifier;

#[component]
pub fn Footer() -> Element {
    let mut i18n = i18n();
    let mut change_language = move |lang: LanguageIdentifier| i18n.set_language(lang);
    let mut mode = use_signal(|| None);

    document::eval(
        r#"let theme = (localStorage.theme ==='dark' || (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)) ? 'dark' : 'light';
        document.documentElement.setAttribute('data-theme', theme);"#,
    );

    use_effect(move || match mode() {
        Some(theme_mode) => {
            document::eval(&format!(
                r#"window.localStorage.setItem('theme', '{theme_mode}');
                document.documentElement.setAttribute('data-theme', '{theme_mode}');"#
            ));
        }
        None => {}
    });

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
                                    {tid!("menu.home")}
                                }
                            },
                            li {
                                class: "my-2",
                                Link {
                                    class: "hover:text-gray-700 dark:hover:text-gray-300",
                                    to: Route::Product {},
                                    {tid!("menu.product")}
                                }
                            },
                            li {
                                class: "my-2",
                                Link {
                                    class: "hover:text-gray-700 dark:hover:text-gray-300",
                                    to: Route::Pricing {},
                                    {tid!("menu.pricing")}
                                }
                            },
                            li {
                                class: "my-2",
                                Link {
                                    class: "hover:text-gray-700 dark:hover:text-gray-300",
                                    to: Route::Contact {},
                                    {tid!("menu.contact")}
                                }
                            },
                        }
                    },

                    div {
                        class: "flex-1",
                        span {
                            class: "font-bold",
                            {tid!("menu-support")}
                        },
                        ul {
                            li {
                                class: "my-2",
                                Link {
                                    class: "hover:text-gray-700 dark:hover:text-gray-300",
                                    to: "#",
                                    {tid!("menu-support.faq")}
                                }
                            },
                            li {
                                class: "my-2",
                                Link {
                                    class: "hover:text-gray-700 dark:hover:text-gray-300",
                                    to: "#",
                                    {tid!("menu-support.tutorials")}
                                }
                            },
                            li {
                                class: "my-2",
                                Link {
                                    class: "hover:text-gray-700 dark:hover:text-gray-300",
                                    to: "#",
                                    {tid!("menu-support.request")}
                                }
                            },
                        }
                    }
                }
            },
            div {
                class: "flex flex-wrap justify-center my-8",
                div {
                    onclick: move |_| change_language("en".parse().expect("No 'en' language")),
                    class: "inline-block p-2 cursor-pointer",
                    "en"
                },
                div {
                    onclick: move |_| change_language("fr".parse().expect("No 'fr' language")),
                    class: "inline-block p-2 cursor-pointer",
                    "fr"
                }
            },
            div {
                class: "flex flex-wrap justify-center my-8",
                div {
                    onclick: move |_| mode.set(Some("light")),
                    class: "inline-block p-2 cursor-pointer",
                    "light"
                },
                div {
                    onclick: move |_| mode.set(Some("dark")),
                    class: "inline-block p-2 cursor-pointer",
                    "dark"
                }
            },
            div {
                class: "flex justify-center items-center my-8",
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
