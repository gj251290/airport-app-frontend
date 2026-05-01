use dioxus::prelude::*;
use crate::api::flights::get_flights;   // función a implementar en api/flights.rs
use crate::models::flight::Flight;      // struct Flight que mapea el backend
use crate::api::client::ApiClient;

pub fn FlightResults(cx: Scope) -> Element {
    // Hook para cargar resultados de vuelos
    let flights = use_future(cx, (), |_| async move {
        let client = ApiClient::new().await;
        get_flights(&client).await.unwrap_or_default()
    });

    cx.render(rsx! {
        div { class: "results-page",
            h1 { class: "results-header", "Flight Results" }

            // Filtros (ejemplo básico)
            div { class: "results-filters",
                input { placeholder: "Origin" }
                input { placeholder: "Destination" }
                input { placeholder: "Date" }
                button { "Search" }
            }

            match flights.value() {
                Some(list) => rsx! {
                    div { class: "results-list",
                        list.iter().map(|flight| rsx! {
                            div { class: "result-card",
                                h2 { "{flight.origin} → {flight.destination}" }
                                p { "Flight: {flight.code}" }
                                p { "Airline: {flight.airline}" }
                                p { "Departure: {flight.departure_time}" }
                                p { "Arrival: {flight.arrival_time}" }
                                p { "Price: ${flight.price}" }
                                button { "Book Now" }
                            }
                        })
                    }
                },
                None => rsx! { p { "Loading flights..." } }
            }
        }
    })
}
