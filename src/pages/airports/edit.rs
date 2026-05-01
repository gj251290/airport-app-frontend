use dioxus::prelude::*;
use crate::api::airports::update_airport;
use crate::models::airport::Airport;
use crate::api::client::ApiClient;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub fn AirportEdit(cx: Scope, airport: Airport) -> Element {
    let name = use_state(cx, || airport.name.clone());
    let code = use_state(cx, || airport.code.clone());
    let city = use_state(cx, || airport.city.clone().unwrap_or_default());
    let country = use_state(cx, || airport.country.clone());

    let on_submit = {
        let name = name.clone();
        let code = code.clone();
        let city = city.clone();
        let country = country.clone();
        let id = airport.id;
        let created_at = airport.created_at;

        move |_| {
            cx.spawn({
                let name = name.get().clone();
                let code = code.get().clone();
                let city = city.get().clone();
                let country = country.get().clone();

                async move {
                    let client = ApiClient::new().await;
                    let updated = Airport {
                        id,
                        code,
                        name,
                        city: Some(city),
                        country,
                        created_at,
                    };
                    let _ = update_airport(&client, id, &updated).await;
                }
            });
        }
    };

    cx.render(rsx! {
        div { class: "form-container",
            h1 { "Edit Airport" }
            input { value: "{name}", oninput: move |e| name.set(e.value.clone()), placeholder: "Name" }
            input { value: "{code}", oninput: move |e| code.set(e.value.clone()), placeholder: "Code" }
            input { value: "{city}", oninput: move |e| city.set(e.value.clone()), placeholder: "City" }
            input { value: "{country}", oninput: move |e| country.set(e.value.clone()), placeholder: "Country" }
            button { onclick: on_submit, "Update" }
        }
    })
}
