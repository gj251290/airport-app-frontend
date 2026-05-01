use dioxus::prelude::*;
use crate::api::airports::get_airports;
use crate::models::airport::Airport;
use crate::api::client::ApiClient;

pub fn AirportsList(cx: Scope) -> Element {
    let airports = use_future(cx, (), |_| async move {
        let client = ApiClient::new().await;
        get_airports(&client).await.unwrap_or_default()
    });

    cx.render(rsx! {
        div { class: "airports-page",
            h1 { class: "airports-header", "Airports" }

            match airports.value() {
                Some(list) => rsx! {
                    div { class: "airports-list",
                        list.iter().map(|airport| rsx! {
                            div { class: "airport-card",
                                h2 { "{airport.name}" }
                                p { "Code: {airport.code}" }
                                p { "City: {airport.city.clone().unwrap_or_default()}" }
                                p { "Country: {airport.country}" }
                            }
                        })
                    }
                },
                None => rsx! { p { "Loading airports..." } }
            }
        }
    })
}