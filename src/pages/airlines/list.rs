use dioxus::prelude::*;
use crate::api::airlines::get_airlines;
use crate::models::airline::Airline;
use crate::api::client::ApiClient;

pub fn AirlinesList(cx: Scope) -> Element {
    let airlines = use_future(cx, (), |_| async move {
        let client = ApiClient::new().await;
        get_airlines(&client).await.unwrap_or_default()
    });

    cx.render(rsx! {
        div { class: "airlines-page",
            h1 { class: "airlines-header", "Airlines" }

            match airlines.value() {
                Some(list) => rsx! {
                    table { class: "airlines-table",
                        thead {
                            tr {
                                th { "Code" }
                                th { "Name" }
                                th { "Country" }
                            }
                        }
                        tbody {
                            list.iter().map(|airline| rsx! {
                                tr {
                                    td { "{airline.code}" }
                                    td { "{airline.name}" }
                                    td { "{airline.country.clone().unwrap_or_default()}" }
                                }
                            })
                        }
                    }
                },
                None => rsx! { p { "Loading airlines..." } }
            }
        }
    })
}