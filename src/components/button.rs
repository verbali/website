use crate::Route;
use dioxus::prelude::*;

const LOGO: Asset = asset!("/assets/logo.svg");

#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    label: String,
    route: Option<Route>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let class =
        "bg-blue-700 dark:bg-blue-800 hover:bg-blue-800 dark:hover:bg-blue-700 font-bold text-white text-xl font-fredoka py-4 px-8 rounded-lg cursor-pointer";

    if let Some(route) = props.route {
        return rsx! {
            Link {
                class: class,
                to: route,
                "{props.label}"
            }
        };
    } else {
        rsx! {
            button {
                class: class,
                "{props.label}"
            }
        }
    }
}
