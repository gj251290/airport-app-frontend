use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct CardProps {
    pub image_url: String,
    pub location: String,
    pub country: String,
    pub price: usize,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    const CSS: Asset = asset!("/assets/components/card.css");

    rsx! {
        document::Stylesheet { href: CSS }

        div { class: "destination-card",
            div { class: "destination-card__image-wrapper",
                img { src: "{props.image_url}", alt: "{props.location}", class: "destination-card__image" }
                div { class: "destination-card__overlay" }
                div { class: "destination-card__badge",
                    span { class: "destination-card__country", "{props.country}" }
                    h3 { class: "destination-card__city", "{props.location}" }
                }
            }
            div { class: "destination-card__footer",
                span { class: "destination-card__label", "Viaje de ida y vuelta desde" }
                span { class: "destination-card__price", "${props.price}" }
            }
        }
    }
}
