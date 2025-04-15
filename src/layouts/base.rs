use crate::components::layouts::{Footer, Navbar};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn BaseLayout() -> Element {
    rsx! {
        Navbar {}
        Outlet::<Route> {}
        Footer {}
    }
}
