use crate::Route;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    label: String,
    route: Option<Route>,
    r#type: Option<String>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let class =
        "inline-block bg-blue-700 dark:bg-blue-800 hover:bg-blue-800 dark:hover:bg-blue-700 font-bold text-white text-xl font-fredoka py-4 px-8 rounded-lg cursor-pointer";

    match props.route {
        Some(route) => {
            rsx! {
                Link {
                    class: class,
                    to: route,
                    "{props.label}"
                }
            }
        }
        None => {
            rsx! {
                button {
                    class: class,
                    type: props.r#type.unwrap_or("button".to_string()),
                    "{props.label}"
                }
            }
        }
    }
}
