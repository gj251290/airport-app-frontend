use dioxus::prelude::*;

#[component]
pub fn NewsletterForm() -> Element {
    const CSS: Asset = asset!("/assets/css/components/form.css");

    rsx! {
        document::Stylesheet { href: CSS }

        div { class: "newsletter",
            div { class: "newsletter__content",
                h2 { class: "newsletter__title", "Únete a los viajeros de élite" }
                p { class: "newsletter__text",
                    "Suscríbete para recibir acceso exclusivo a tarifas con errores y promociones de temporada de aerolíneas directamente en tu correo electrónico."
                }
            }
            div { class: "newsletter__form",
                input { class: "newsletter__input", placeholder: "Introduce tu correo electrónico", r#type: "email" }
                button { class: "newsletter__button", "Únete ahora" }
            }
        }
    }
}
