use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use unic_langid::langid;
use verbali_design_system::assets::{DS_CSS, LOGO_SVG};

use layouts::BaseLayout;
use views::{Contact, Home, Pricing, Product};

mod components;
mod helpers;
mod layouts;
mod views;

#[cfg(feature = "server")]
mod models;
#[cfg(feature = "server")]
mod schema;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(BaseLayout)]
        #[route("/")]
        Home {},

        #[route("/product")]
        Product {},

        #[route("/pricing")]
        Pricing {},

        #[route("/contact")]
        Contact {},
}

const MAIN_CSS: Asset = asset!("/assets/style.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_init_i18n(|| {
        I18nConfig::new(langid!("fr"))
            .with_locale((langid!("fr"), include_str!("./locales/fr.ftl")))
            .with_locale((langid!("en"), include_str!("./locales/en.ftl")))
    });

    rsx! {
        document::Link { rel: "icon", href: LOGO_SVG }
        document::Link { rel: "stylesheet", href: DS_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}
