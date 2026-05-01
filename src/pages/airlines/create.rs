use dioxus::prelude::*;
use crate::api::airlines::create_airline;
use crate::models::airline::Airline;
use crate::api::client::ApiClient;
use uuid::Uuid;
use chrono::Utc;

pub fn AirlineCreate(cx: Scope) -> Element {
    let name = use_state(cx, || "".to_string());
    let code = use_state(cx, || "".to_string());
    let country = use_state(cx, || "".to_string());

    let on_submit = {
        let name = name.clone();
        let code = code.clone();
        let country = country.clone();

        move |_| {
            cx.spawn({
                let name = name.get().clone();
                let code = code.get().clone();
                let country = country.get().clone();

                async move {
                    let client = ApiClient::new().await;
                    let airline = Airline {
                        id: Uuid::new_v4(),
                        code,
                        name,
                        country: Some(country),
                        created_at: Utc::now(),
                    };
                    let _ = create_airline(&client, &airline).await;
                }
            });
        }
    };

    cx.render(rsx! {
        div { class: "form-container",
            h1 { "Create Airline" }
            input { value: "{name}", oninput: move |e| name.set(e.value.clone()), placeholder: "Name" }
            input { value: "{code}", oninput: move |e| code.set(e.value.clone()), placeholder: "Code" }
            input { value: "{country}", oninput: move |e| country.set(e.value.clone()), placeholder: "Country" }
            button { onclick: on_submit, "Save" }
        }
    })
}