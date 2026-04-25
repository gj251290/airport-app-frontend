// src/components/passenger.rs
use crate::models::search::PassengerCount;
use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/css/components/passenger.css");

#[derive(Props, Clone, PartialEq)]
pub struct PassengerSelectorProps {
    pub passengers: PassengerCount,
    pub on_change: EventHandler<PassengerCount>,
    pub error: Option<String>,
}

#[component]
pub fn PassengerSelector(props: PassengerSelectorProps) -> Element {
    let mut show_popover = use_signal(|| false);
    let mut local_passengers = use_signal(|| props.passengers.clone());

    // Sincronizar cuando cambie la prop externa
    use_effect(move || {
        local_passengers.set(props.passengers.clone());
    });

    let display_text = if local_passengers().adults == 1 {
        format!("{} Adulto", local_passengers().adults)
    } else {
        format!("{} Adultos", local_passengers().adults)
    };

    let on_change_handler = props.on_change.clone();

    rsx! {
        document::Stylesheet { href: CSS }

        div { class: "passenger-selector",
            label { class: "passenger-selector__label", "Pasajeros" }
            button {
                class: "passenger-selector__button",
                onclick: move |event: Event<MouseData>| {
                    event.stop_propagation();
                    show_popover.set(!show_popover());
                },
                "{display_text}"
            }
            if show_popover() {
                // Overlay para cerrar al hacer clic fuera
                div {
                    class: "popover-overlay",
                    onclick: move |_| show_popover.set(false),
                }
                div {
                    class: "passenger-selector__popover",
                    // Adultos
                    div { class: "passenger-selector__row",
                        div { class: "passenger-selector__info",
                            span { class: "passenger-selector__type", "Adultos" }
                            span { class: "passenger-selector__desc", "12+ años" }
                        }
                        div { class: "passenger-selector__controls",
                            button {
                                onclick: {
                                    let mut local = local_passengers.clone();
                                    let handler = on_change_handler.clone();
                                    move |_| {
                                        let mut new = local();
                                        if new.adults > 1 { new.adults -= 1; }
                                        if new.infants > new.adults { new.infants = new.adults; }
                                        local.set(new.clone());
                                        handler.call(new);
                                    }
                                },
                                "-"
                            }
                            span { "{local_passengers().adults}" }
                            button {
                                onclick: {
                                    let mut local = local_passengers.clone();
                                    let handler = on_change_handler.clone();
                                    move |_| {
                                        let mut new = local();
                                        new.adults += 1;
                                        if new.infants > new.adults { new.infants = new.adults; }
                                        local.set(new.clone());
                                        handler.call(new);
                                    }
                                },
                                "+"
                            }
                        }
                    }
                    // Niños
                    div { class: "passenger-selector__row",
                        div { class: "passenger-selector__info",
                            span { class: "passenger-selector__type", "Niños" }
                            span { class: "passenger-selector__desc", "2-11 años" }
                        }
                        div { class: "passenger-selector__controls",
                            button {
                                onclick: {
                                    let mut local = local_passengers.clone();
                                    let handler = on_change_handler.clone();
                                    move |_| {
                                        let mut new = local();
                                        if new.children > 0 { new.children -= 1; }
                                        if new.infants > new.adults { new.infants = new.adults; }
                                        local.set(new.clone());
                                        handler.call(new);
                                    }
                                },
                                "-"
                            }
                            span { "{local_passengers().children}" }
                            button {
                                onclick: {
                                    let mut local = local_passengers.clone();
                                    let handler = on_change_handler.clone();
                                    move |_| {
                                        let mut new = local();
                                        new.children += 1;
                                        if new.infants > new.adults { new.infants = new.adults; }
                                        local.set(new.clone());
                                        handler.call(new);
                                    }
                                },
                                "+"
                            }
                        }
                    }
                    // Bebés
                    div { class: "passenger-selector__row",
                        div { class: "passenger-selector__info",
                            span { class: "passenger-selector__type", "Bebés" }
                            span { class: "passenger-selector__desc", "Menores de 2 años" }
                        }
                        div { class: "passenger-selector__controls",
                            button {
                                onclick: {
                                    let mut local = local_passengers.clone();
                                    let handler = on_change_handler.clone();
                                    move |_| {
                                        let mut new = local();
                                        if new.infants > 0 { new.infants -= 1; }
                                        if new.infants > new.adults { new.infants = new.adults; }
                                        local.set(new.clone());
                                        handler.call(new);
                                    }
                                },
                                "-"
                            }
                            span { "{local_passengers().infants}" }
                            button {
                                onclick: {
                                    let mut local = local_passengers.clone();
                                    let handler = on_change_handler.clone();
                                    move |_| {
                                        let mut new = local();
                                        new.infants += 1;
                                        if new.infants > new.adults { new.infants = new.adults; }
                                        local.set(new.clone());
                                        handler.call(new);
                                    }
                                },
                                "+"
                            }
                        }
                    }
                    p { class: "passenger-selector__note", "Máximo 1 bebé por adulto" }
                }
            }
            if let Some(err) = &props.error {
                p { class: "passenger-selector__error", "{err}" }
            }
        }
    }
}
