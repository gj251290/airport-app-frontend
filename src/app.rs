use dioxus::prelude::*;

use crate::pages::landing::Landing;

const GLOBAL_CSS: Asset = asset!("/assets/css/main.css");
const VARIABLES_CSS: Asset = asset!("/assets/css/variables.css");

#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    Landing {},
    // Otras rutas se agregarán después
}

pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: VARIABLES_CSS }
        document::Stylesheet { href: GLOBAL_CSS }

        Router::<Route> {}
    }
}
