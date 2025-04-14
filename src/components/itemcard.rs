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
            class: "flex flex-row items-center my-16",

            div {
                img {
                    class: "w-16 h-16 mr-8",
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
