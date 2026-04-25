use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdPlaneLanding, LdPlaneTakeoff, LdSearch};
use dioxus_free_icons::Icon;

use crate::components::airport_autocomplete::AirportAutocomplete;
use crate::components::datepicker::DateRangePicker;
use crate::components::passenger::PassengerSelector;
use crate::models::search::{PassengerCount, SearchParams, TripType};

#[component]
pub fn SearchWidget() -> Element {
    const CSS: Asset = asset!("/assets/css/components/search.css");

    let mut trip_type = use_signal(|| TripType::RoundTrip);
    let mut origin = use_signal(|| None::<String>);
    let mut destination = use_signal(|| None::<String>);
    let mut depart_date = use_signal(|| None::<String>);
    let mut return_date = use_signal(|| None::<String>);
    let mut passengers = use_signal(PassengerCount::default);

    let mut origin_error = use_signal(|| None::<String>);
    let mut dest_error = use_signal(|| None::<String>);
    let mut date_error = use_signal(|| None::<String>);

    let mut validate_origin_dest = move || {
        if let (Some(orig), Some(dest)) = (origin(), destination()) {
            if orig == dest {
                origin_error.set(Some("Origen y destino no pueden ser iguales".to_string()));
                dest_error.set(Some("Origen y destino no pueden ser iguales".to_string()));
            } else {
                origin_error.set(None);
                dest_error.set(None);
            }
        } else {
            origin_error.set(None);
            dest_error.set(None);
        }
    };

    use_effect(move || {
        validate_origin_dest();
    });

    let mut on_trip_type_change = move |ty: TripType| {
        trip_type.set(ty.clone());
        if matches!(ty, TripType::OneWay) {
            return_date.set(None);
        }
    };

    let on_submit = move |_| {
        let mut has_error = false;
        if origin().is_none() {
            origin_error.set(Some("Seleccione un origen".to_string()));
            has_error = true;
        }
        if destination().is_none() {
            dest_error.set(Some("Seleccione un destino".to_string()));
            has_error = true;
        }
        if depart_date().is_none() {
            date_error.set(Some("Seleccione fecha de ida".to_string()));
            has_error = true;
        } else if let (TripType::RoundTrip, Some(dep), Some(ret)) =
            (trip_type(), depart_date(), return_date())
        {
            if ret <= dep {
                date_error.set(Some(
                    "Fecha de vuelta debe ser posterior a la ida".to_string(),
                ));
                has_error = true;
            }
        }
        if has_error {
            return;
        }

        let params = SearchParams {
            trip_type: trip_type(),
            origin: origin(),
            destination: destination(),
            depart_date: depart_date(),
            return_date: if matches!(trip_type(), TripType::RoundTrip) {
                return_date()
            } else {
                None
            },
            passengers: passengers(),
        };
        println!("Buscando con parámetros: {:?}", params);
    };

    rsx! {
        document::Stylesheet { href: CSS }

        div { class: "search-widget",
            div { class: "search-widget__tabs",
                button {
                    class: if matches!(trip_type(), TripType::RoundTrip) { "search-widget__tab--active" } else { "search-widget__tab" },
                    onclick: move |_| on_trip_type_change(TripType::RoundTrip),
                    Icon { width: 16, height: 16, icon: LdPlaneTakeoff }
                    span { "Ida y vuelta" }
                }
                button {
                    class: if matches!(trip_type(), TripType::OneWay) { "search-widget__tab--active" } else { "search-widget__tab" },
                    onclick: move |_| on_trip_type_change(TripType::OneWay),
                    Icon { width: 16, height: 16, icon: LdPlaneLanding }
                    span { "Solo ida" }
                }
            }

            div { class: "search-widget__form",
                AirportAutocomplete {
                    label: "Desde".to_string(),
                    placeholder: "Ciudad de salida".to_string(),
                    value: origin(),
                    on_select: move |code| {
                        origin.set(Some(code));
                        validate_origin_dest();
                    },
                    error: origin_error(),
                }
                AirportAutocomplete {
                    label: "Hasta".to_string(),
                    placeholder: "Ciudad de llegada".to_string(),
                    value: destination(),
                    on_select: move |code| {
                        destination.set(Some(code));
                        validate_origin_dest();
                    },
                    error: dest_error(),
                }
                DateRangePicker {
                    depart_date: depart_date(),
                    return_date: return_date(),
                    on_depart_select: move |date| {
                        depart_date.set(Some(date));
                        date_error.set(None);
                    },
                    on_return_select: move |date| {
                        return_date.set(Some(date));
                        date_error.set(None);
                    },
                    show_return: matches!(trip_type(), TripType::RoundTrip),
                    error: date_error(),
                }
                PassengerSelector {
                    passengers: passengers(),
                    on_change: move |p| passengers.set(p),
                    error: None,
                }
            }

            div { class: "search-widget__action",
                button { class: "search-widget__button", onclick: on_submit,
                    span { "Buscar vuelos" }
                    Icon { width: 20, height: 20, icon: LdSearch }
                }
            }
        }
    }
}
