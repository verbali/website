use crate::components::button::Button;
use crate::components::forms::inputs::{InputText, Textarea};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! {
        div {
            class: "mx-auto w-4xl",

            div {
                class: "my-24 text-center",
                h1 {
                    class: "font-bold text-6xl my-8",
                    "Contact"
                }
                p {
                    class: "my-8 text-xl",
                    "Une question ? Envoyez nous un message."
                }
            }

            form {
                InputText {
                    label: "Nom",
                    placeholder: "John Doe",
                    required: true
                }
                InputText {
                    label: "Email",
                    placeholder: "John@doe.com",
                    required: true
                }
                Textarea {
                    label: "Message",
                    required: true
                }
                div {
                    class: "text-center",
                    Button {
                        label: "Envoyer",
                        type: "submit"
                    }
                }
            }
        }
    }
}
