use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;

const CSS: Asset = asset!("/assets/css/components/autocomplete.css");

// Datos de ejemplo - en producción vendrían de la API
const AIRPORTS: &[(&str, &str)] = &[
    ("BOG", "Bogotá - El Dorado"),
    ("MIA", "Miami - International"),
    ("JFK", "New York - JFK"),
    ("LHR", "London - Heathrow"),
    ("CDG", "Paris - Charles de Gaulle"),
    ("NRT", "Tokyo - Narita"),
    ("SYD", "Sydney - Kingsford Smith"),
    ("FCO", "Rome - Fiumicino"),
    ("MEX", "Mexico City - Benito Juárez"),
    ("GRU", "São Paulo - Guarulhos"),
];

#[derive(Props, Clone, PartialEq)]
pub struct AirportAutocompleteProps {
    pub label: String,
    pub placeholder: String,
    pub value: Option<String>,
    pub on_select: EventHandler<String>,
    pub error: Option<String>,
}

#[component]
pub fn AirportAutocomplete(props: AirportAutocompleteProps) -> Element {
    let mut input_value = use_signal(|| props.value.clone().unwrap_or_default());
    let mut suggestions = use_signal(Vec::<(String, String)>::new);
    let mut show_suggestions = use_signal(|| false);

    // Actualizar el valor interno cuando cambia la prop externa
    use_effect(move || {
        input_value.set(props.value.clone().unwrap_or_default());
    });

    let on_input = move |evt: Event<FormData>| {
        let val = evt.value();
        input_value.set(val.clone());

        if val.is_empty() {
            suggestions.set(vec![]);
            show_suggestions.set(false);
        } else {
            let filtered: Vec<_> = AIRPORTS
                .iter()
                .filter(|(code, name)| {
                    code.to_lowercase().contains(&val.to_lowercase())
                        || name.to_lowercase().contains(&val.to_lowercase())
                })
                .map(|(code, name)| (code.to_string(), name.to_string()))
                .collect();

            let has_suggestions = !filtered.is_empty();
            suggestions.set(filtered);
            show_suggestions.set(has_suggestions);
        }
    };

    let on_blur = move |_| {
        let mut show = show_suggestions.clone();
        spawn_local(async move {
            TimeoutFuture::new(200).await;
            show.set(false);
        });
    };

    let mut on_select_suggestion = move |code: String, name: String| {
        input_value.set(format!("{} - {}", code, name));
        show_suggestions.set(false);
        props.on_select.call(code);
    };

    rsx! {
        document::Stylesheet { href: CSS }

        div { class: "autocomplete",
            label { class: "autocomplete__label", "{props.label}" }
            div { class: "autocomplete__wrapper",
                input {
                    r#type: "text",
                    class: "autocomplete__input",
                    placeholder: props.placeholder,
                    value: "{input_value}",
                    oninput: on_input,
                    onblur: on_blur,
                }
                if show_suggestions() {
                    div { class: "autocomplete__suggestions",
                        for (code, name) in suggestions() {
                            div {
                                class: "autocomplete__suggestion",
                                onclick: move |_| on_select_suggestion(code.clone(), name.clone()),
                                span { class: "autocomplete__code", "{code}" }
                                span { class: "autocomplete__name", "{name}" }
                            }
                        }
                    }
                }
            }
            if let Some(err) = &props.error {
                p { class: "autocomplete__error", "{err}" }
            }
        }
    }
}
