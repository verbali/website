use dioxus::prelude::*;

use layouts::Navbar;
use views::{Contact, Home, Pricing, Product};

mod components;
mod layouts;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},

        #[route("/product")]
        Product {},

        #[route("/pricing")]
        Pricing {},

        #[route("/contact")]
        Contact {},
}

const FAVICON: Asset = asset!("/assets/logo.svg");
const MAIN_CSS: Asset = asset!("/assets/style.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
