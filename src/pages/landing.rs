use crate::components::card::Card;
use crate::components::footer::Footer;
use crate::components::form::NewsletterForm;
use crate::components::navbar::Navbar;
use crate::components::search::SearchWidget;
use dioxus::prelude::*;

use dioxus_free_icons::icons::ld_icons::LdArrowRight;
use dioxus_free_icons::Icon;

#[component]
pub fn Landing() -> Element {
    const CSS: Asset = asset!("/assets/css/pages/landing.css");

    rsx! {
        document::Stylesheet { href: CSS }

        div { class: "landing",
            Navbar {}
            main { class: "landing__main",

                section { class: "hero",
                    div { class: "hero__background" }
                    div { class: "hero__overlay" }
                    div { class: "hero__container",
                        div { class: "hero__content",
                            h1 { class: "hero__title", "Encuentra tu próxima aventura" }
                            p { class: "hero__subtitle",
                                "Reservas de vuelos premium con precisión editorial. Tu viaje comienza en el horizonte."
                            }
                        }
                        SearchWidget {}
                    }
                }

                section { class: "destinations",
                    div { class: "destinations__header",
                        div { class: "destinations__header-left",
                            span { class: "destinations__eyebrow", "Viajes seleccionados" }
                            h2 { class: "destinations__title", "Destinos de moda" }
                        }
                        a { class: "destinations__link", href: "#",
                            span { "Ver todos los destinos" }
                            span {
                                Icon {
                                    icon: LdArrowRight,
                                    width: 20,
                                    height: 20,
                                    class: "arrow__right",
                                }
                            }
                        }
                    }
                    div { class: "destinations__grid",
                        Card {
                            image_url: "https://images.unsplash.com/photo-1570077188670-e3a8d69ac5ff?w=800&h=600&fit=crop".to_string(),
                            location: "Amalfi".to_string(),
                            country: "Italia".to_string(),
                            price: 520,
                        },
                        Card {
                            image_url: "https://images.unsplash.com/photo-1506973035872-a4ec16b8e8d9?w=800&h=600&fit=crop".to_string(),
                            location: "Sídney".to_string(),
                            country: "Australia".to_string(),
                            price: 890,
                        },
                        Card {
                            image_url: "https://images.unsplash.com/photo-1526481280693-3bfa7568e0f3?w=800&h=600&fit=crop".to_string(),
                            location: "Narita".to_string(),
                            country: "Japón".to_string(),
                            price: 1100,
                        },
                    }
                }

                section { class: "newsletter-section",
                    NewsletterForm {}
                }
            }

            Footer {}
        }
    }
}
