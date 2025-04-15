use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ItemCardProps {
    icon: Asset,
    title: String,
    content: String,
}

#[component]
pub fn ItemCard(props: ItemCardProps) -> Element {
    rsx! {
        div {
            class: "sm:flex sm:flex-row sm:items-center my-16",

            div {
                img {
                    class: "w-16 h-16 mx-auto sm:mx-8",
                    src: "{props.icon}",
                    alt: "{props.title} icon"
                }
            }

            div {
                h2 {
                    class: "font-bold text-4xl",
                    "{props.title}"
                }
                p {
                    class: "text-xl",
                    "{props.content}"
                }
            }
        }
    }
}
