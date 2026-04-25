use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{
    LdCalendar, LdPlaneLanding, LdPlaneTakeoff, LdSearch, LdUsers,
};
use dioxus_free_icons::Icon;

#[component]
pub fn SearchWidget() -> Element {
    const CSS: Asset = asset!("/assets/components/search.css");

    rsx! {
        document::Stylesheet { href: CSS }

        div { class: "search-widget",
            div { class: "search-widget__tabs",
                button { class: "search-widget__tab search-widget__tab--active",
                    Icon { width: 16, height: 16, icon: LdPlaneTakeoff, class: "search-widget__icon" }
                    span { "Ida y vuelta" }
                }
                button { class: "search-widget__tab",
                    Icon { width: 16, height: 16, icon: LdPlaneLanding, class: "search-widget__icon" }
                    span { "Solo ida" }
                }
            }
            div { class: "search-widget__form",
                div { class: "search-widget__field",
                    label { class: "search-widget__label", "Desde" }
                    div { class: "search-widget__input-wrapper",
                        Icon { width: 18, height: 18, icon: LdPlaneTakeoff, class: "search-widget__input-icon" }
                        input { class: "search-widget__input", placeholder: "Ciudad de salida", r#type: "text" }
                    }
                }
                div { class: "search-widget__field",
                    label { class: "search-widget__label", "To" }
                    div { class: "search-widget__input-wrapper",
                        Icon { width: 18, height: 18, icon: LdPlaneLanding, class: "search-widget__input-icon" }
                        input { class: "search-widget__input", placeholder: "Ciudad de llegada", r#type: "text" }
                    }
                }
                div { class: "search-widget__field",
                    label { class: "search-widget__label", "Fechas" }
                    div { class: "search-widget__input-wrapper",
                        Icon { width: 18, height: 18, icon: LdCalendar, class: "search-widget__input-icon" }
                        input { class: "search-widget__input", placeholder: "Seleccione las fechas", r#type: "text" }
                    }
                }
                div { class: "search-widget__field",
                    label { class: "search-widget__label", "Pasajeros" }
                    div { class: "search-widget__input-wrapper",
                        Icon { width: 18, height: 18, icon: LdUsers, class: "search-widget__input-icon" }
                        input { class: "search-widget__input", placeholder: "1 Adulto", r#type: "text" }
                    }
                }
            }
            div { class: "search-widget__action",
                button { class: "search-widget__button",
                    span { "Buscar vuelos" }
                    Icon { width: 20, height: 20, icon: LdSearch }
                }
            }
        }
    }
}
