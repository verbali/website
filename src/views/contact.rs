use crate::components::button::Button;
use crate::components::forms::inputs::{InputText, Textarea};
use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn Contact() -> Element {
    rsx! {
        div {
            class: "max-w-4xl lg:mx-auto lg:w-4xl p-4",

            div {
                class: "my-24 text-center",
                h1 {
                    class: "font-bold text-6xl my-8",
                    {tid!("contact")}
                }
                p {
                    class: "my-8 text-xl",
                    {tid!("contact.content")}
                }
            }

            form {
                InputText {
                    label: tid!("contact.name"),
                    placeholder: "John Doe",
                    required: true
                }
                InputText {
                    label: tid!("contact.mail"),
                    placeholder: "John@doe.com",
                    required: true
                }
                Textarea {
                    label: tid!("contact.message"),
                    required: true
                }
                div {
                    class: "text-center",
                    Button {
                        label: tid!("contact.submit"),
                        type: "submit"
                    }
                }
            }
        }
    }
}
