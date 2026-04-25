use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    const CSS: Asset = asset!("/assets/components/navbar.css");

    rsx! {
        document::Stylesheet { href: CSS }

        header { class: "navbar",
            div { class: "navbar__container",
                div { class: "navbar__logo", "SkyBook" }
                nav { class: "navbar__links",
                    a { class: "navbar__link navbar__link--active", href: "#", "Buscar vuelos" }
                    a { class: "navbar__link", href: "#", "Mis reservas" }
                    a { class: "navbar__link", href: "#", "Registrarse" }
                    a { class: "navbar__link", href: "#", "Ayuda" }
                }
                div { class: "navbar__actions",
                    button { class: "navbar__support", "Soporte" }
                    button { class: "navbar__signin", "Iniciar sesión" }
                }
            }
        }
    }
}
