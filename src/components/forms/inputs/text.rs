use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct InputTextProps {
    label: String,
    placeholder: Option<String>,
    required: Option<bool>,
}

#[component]
pub fn InputText(props: InputTextProps) -> Element {
    let mut label = props.label;

    if let Some(_) = props.required {
        label.push_str(" *");
    }

    rsx! {
        div {
            class: "mb-8",

            label {
                class: "block mb-2 font-medium text-gray-900 dark:text-white",
                "{label}"
            }
            input {
                class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 outline-none",
                required: match props.required {
                    Some(r) => r,
                    None => false,
                },
                placeholder: match props.placeholder {
                    Some(p) => p,
                    None => "".to_string(),
                },
            }
        }
    }
}
