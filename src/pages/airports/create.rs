use dioxus::prelude::*;
use crate::api::airports::create_airport; // Función de API para crear aeropuerto
use crate::models::airport::Airport;      // Struct Airport
use crate::api::client::ApiClient;
use uuid::Uuid;
use chrono::Utc;


pub fn AirportCreate(cx: Scope) -> Element {
    let name = use_state(cx, || "".to_string());
    let code = use_state(cx, || "".to_string());
    let city = use_state(cx, || "".to_string());
    let country = use_state(cx, || "".to_string());

    let on_submit = {
        let name = name.clone();
        let code = code.clone();
        let city = city.clone();
        let country = country.clone();

        move |_| {
            cx.spawn({
                let name = name.get().clone();
                let code = code.get().clone();
                let city = city.get().clone();
                let country = country.get().clone();

                async move {
                    let client = ApiClient::new().await;
                    let airport = Airport {
                        id: Uuid::new_v4(),
                        code,
                        name,
                        city: Some(city),
                        country,
                        created_at: Utc::now(),
                    };
                    let _ = create_airport(&client, &airport).await;
                }
            });
        }
    };

    cx.render(rsx! {
        div { class: "form-container",
            h1 { "Create Airport" }
            input { value: "{name}", oninput: move |e| name.set(e.value.clone()), placeholder: "Name" }
            input { value: "{code}", oninput: move |e| code.set(e.value.clone()), placeholder: "Code" }
            input { value: "{city}", oninput: move |e| city.set(e.value.clone()), placeholder: "City" }
            input { value: "{country}", oninput: move |e| country.set(e.value.clone()), placeholder: "Country" }
            button { onclick: on_submit, "Save" }
        }
    })
}