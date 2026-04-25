// src/components/datepicker.rs
use chrono::{Datelike, Duration, Local, NaiveDate};
use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/css/components/datepicker.css");

#[derive(Props, Clone, PartialEq)]
pub struct DateRangePickerProps {
    pub depart_date: Option<String>,
    pub return_date: Option<String>,
    pub on_depart_select: EventHandler<String>,
    pub on_return_select: EventHandler<String>,
    pub show_return: bool,
    pub error: Option<String>,
}

#[component]
pub fn DateRangePicker(props: DateRangePickerProps) -> Element {
    let mut show_calendar = use_signal(|| false);
    let mut selecting_return = use_signal(|| false);
    let mut current_month = use_signal(|| Local::now().naive_local().date());

    let today = Local::now().naive_local().date();

    let depart_parsed = props
        .depart_date
        .as_ref()
        .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());
    let return_parsed = props
        .return_date
        .as_ref()
        .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

    let on_depart = props.on_depart_select.clone();
    let on_return = props.on_return_select.clone();
    let show_return_flag = props.show_return;

    // Construir los días del calendario
    let days_elements = {
        let year = current_month().year();
        let month = current_month().month();
        let last_day = NaiveDate::from_ymd_opt(year, month + 1, 1)
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap())
            - Duration::days(1);

        let mut elements = Vec::new();
        for day in 1..=last_day.day() {
            let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
            let disabled = date < today;
            let selected = if !selecting_return() {
                depart_parsed == Some(date)
            } else {
                return_parsed == Some(date)
            };

            let mut show_cal = show_calendar.clone();
            let mut select_ret = selecting_return.clone();
            let on_depart_clone = on_depart.clone();
            let on_return_clone = on_return.clone();
            let show_return = show_return_flag;

            let onclick = move |_| {
                if disabled {
                    return;
                }
                if !select_ret() {
                    let date_str = date.format("%Y-%m-%d").to_string();
                    on_depart_clone.call(date_str);
                    if show_return {
                        select_ret.set(true);
                    } else {
                        show_cal.set(false);
                    }
                } else {
                    if let Some(depart) = depart_parsed {
                        if date >= depart {
                            let date_str = date.format("%Y-%m-%d").to_string();
                            on_return_clone.call(date_str);
                            show_cal.set(false);
                            select_ret.set(false);
                        }
                    } else {
                        show_cal.set(false);
                        select_ret.set(false);
                    }
                }
            };

            let class = if selected {
                "calendar__day--selected"
            } else {
                "calendar__day"
            };

            elements.push(rsx! {
                button {
                    class: class,
                    disabled: disabled,
                    onclick: onclick,
                    "{day}"
                }
            });
        }
        elements
    };

    let mut change_month = move |delta: i32| {
        let new_month = current_month() + Duration::days(if delta > 0 { 30 } else { -30 });
        current_month.set(new_month);
    };

    rsx! {
        document::Stylesheet { href: CSS }

        div { class: "date-range-picker",
            div { class: "date-range-picker__fields",
                div { class: "date-range-picker__field",
                    label { class: "date-range-picker__label", "Ida" }
                    input {
                        r#type: "text",
                        class: "date-range-picker__input",
                        placeholder: "YYYY-MM-DD",
                        value: props.depart_date.clone().unwrap_or_default(),
                        readonly: true,
                        onclick: move |event: Event<MouseData>| {
                            event.stop_propagation();
                            selecting_return.set(false);
                            show_calendar.set(!show_calendar());
                        },
                    }
                }
                if props.show_return {
                    div { class: "date-range-picker__field",
                        label { class: "date-range-picker__label", "Vuelta" }
                        input {
                            r#type: "text",
                            class: "date-range-picker__input",
                            placeholder: "YYYY-MM-DD",
                            value: props.return_date.clone().unwrap_or_default(),
                            readonly: true,
                            onclick: move |event: Event<MouseData>| {
                                event.stop_propagation();
                                selecting_return.set(true);
                                show_calendar.set(!show_calendar());
                            },
                        }
                    }
                }
            }
            if show_calendar() {
                // Overlay para cerrar al hacer clic fuera
                div {
                    class: "calendar-overlay",
                    onclick: move |_| {
                        show_calendar.set(false);
                        selecting_return.set(false);
                    },
                }
                div {
                    class: "date-range-picker__calendar",
                    div { class: "calendar__header",
                        button {
                            onclick: move |_| change_month(-1),
                            "<"
                        }
                        span { class: "calendar__month", "{current_month().format(\"%B %Y\")}" }
                        button {
                            onclick: move |_| change_month(1),
                            ">"
                        }
                    }
                    div { class: "calendar__weekdays",
                        span { "L" }, span { "M" }, span { "M" }, span { "J" }, span { "V" }, span { "S" }, span { "D" }
                    }
                    div { class: "calendar__days",
                        { days_elements.into_iter() }
                    }
                }
            }
            if let Some(err) = &props.error {
                p { class: "date-range-picker__error", "{err}" }
            }
        }
    }
}
