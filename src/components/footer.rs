use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/css/components/footer.css");

#[component]
pub fn Footer() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }

        footer { class: "footer",
            div { class: "footer__container",
                div { class: "footer__brand",
                    span { class: "footer__logo", "SkyBook" }
                    p { class: "footer__copyright", "© 2026 SkyBook Aviation. Todos los datos de vuelo están protegidos." }
                }
                div { class: "footer__links",
                    a { href: "#", "Política de privacidad" }
                    a { href: "#", "Condiciones del servicio" }
                    a { href: "#", "Normas" }
                    a { href: "#", "Contacto" }
                }
            }
        }
    }
}
