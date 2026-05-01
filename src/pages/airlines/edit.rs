use dioxus::prelude::*;
use crate::api::airlines::update_airline;
use crate::models::airline::Airline;
use crate::api::client::ApiClient;
use uuid::Uuid;
use chrono::Utc;

pub fn AirlineEdit(cx: Scope, airline: Airline) -> Element {
    let name = use_state(cx, || airline.name.clone());
    let code = use_state(cx, || airline.code.clone());
    let country = use_state(cx, || airline.country.clone().unwrap_or_default());

    let on_submit = {
        let name = name.clone();
        let code = code.clone();
        let country = country.clone();
        let id = airline.id;

        move |_| {
            cx.spawn({
                let name = name.get().clone();
                let code = code.get().clone();
                let country = country.get().clone();

                async move {
                    let client = ApiClient::new().await;
                    let updated = Airline {
                        id,
                        code,
                        name,
                        country: Some(country),
                        created_at: airline.created_at,
                    };
                    let _ = update_airline(&client, id, &updated).await;
                }
            });
        }
    };

    cx.render(rsx! {
        div { class: "form-container",
            h1 { "Edit Airline" }
            input { value: "{name}", oninput: move |e| name.set(e.value.clone()), placeholder: "Name" }
            input { value: "{code}", oninput: move |e| code.set(e.value.clone()), placeholder: "Code" }
            input { value: "{country}", oninput: move |e| country.set(e.value.clone()), placeholder: "Country" }
            button { onclick: on_submit, "Update" }
        }
    })
}