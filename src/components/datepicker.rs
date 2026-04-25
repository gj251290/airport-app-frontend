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

    // Función auxiliar para obtener los días de un mes
    let get_days = |year: i32, month: u32| -> Vec<NaiveDate> {
        let first = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        let last = NaiveDate::from_ymd_opt(year, month + 1, 1)
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap())
            - Duration::days(1);
        (first.day()..=last.day())
            .map(|d| NaiveDate::from_ymd_opt(year, month, d).unwrap())
            .collect()
    };

    // Función para renderizar un mes (devuelve Vec<VNode>)
    let render_month = |year: i32, month: u32| {
        let days = get_days(year, month);
        let first_weekday = days.first().unwrap().weekday().num_days_from_monday();
        let mut cells = Vec::new();

        // celdas vacías iniciales
        for _ in 0..first_weekday {
            cells.push(rsx! { div { class: "calendar__day--empty", "" } });
        }

        for day_date in days {
            let day = day_date.day();
            let disabled = day_date < today;
            let is_depart = depart_parsed == Some(day_date);
            let is_return = return_parsed == Some(day_date);
            let in_range = if let (Some(dep), Some(ret)) = (depart_parsed, return_parsed) {
                day_date > dep && day_date < ret
            } else {
                false
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
                    let date_str = day_date.format("%Y-%m-%d").to_string();
                    on_depart_clone.call(date_str);
                    if show_return {
                        select_ret.set(true);
                    } else {
                        show_cal.set(false);
                    }
                } else {
                    if let Some(dep) = depart_parsed {
                        if day_date >= dep {
                            let date_str = day_date.format("%Y-%m-%d").to_string();
                            on_return_clone.call(date_str);
                            show_cal.set(false);
                            select_ret.set(false);
                        }
                    } else {
                        select_ret.set(false);
                    }
                }
            };

            let class = if is_depart {
                "calendar__day--depart"
            } else if is_return {
                "calendar__day--return"
            } else if in_range {
                "calendar__day--in-range"
            } else {
                "calendar__day"
            };

            cells.push(rsx! {
                button {
                    class: class,
                    disabled: disabled,
                    onclick: onclick,
                    "{day}"
                }
            });
        }

        let month_name = NaiveDate::from_ymd_opt(year, month, 1)
            .unwrap()
            .format("%B %Y")
            .to_string();
        rsx! {
            div { class: "calendar-month",
                div { class: "calendar-month__header", "{month_name}" }
                div { class: "calendar-month__weekdays",
                    span { "L" }, span { "M" }, span { "M" }, span { "J" }, span { "V" }, span { "S" }, span { "D" }
                }
                div { class: "calendar-month__days", { cells.into_iter() } }
            }
        }
    };

    let mut change_month_pair = move |delta: i32| {
        let new = current_month() + Duration::days(if delta > 0 { 30 } else { -30 });
        current_month.set(new);
    };

    let year1 = current_month().year();
    let month1 = current_month().month();
    let next_month = current_month() + Duration::days(30);
    let year2 = next_month.year();
    let month2 = next_month.month();

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
                div {
                    class: "calendar-overlay",
                    onclick: move |_| {
                        show_calendar.set(false);
                        selecting_return.set(false);
                    },
                }
                div { class: "date-range-picker__calendar-container",
                    div { class: "calendar__navigation",
                        button { onclick: move |_| change_month_pair(-1), "←" }
                        button { onclick: move |_| change_month_pair(1), "→" }
                    }
                    div { class: "calendar__months",
                        { render_month(year1, month1) }
                        { render_month(year2, month2) }
                    }
                }
            }
            if let Some(err) = &props.error {
                p { class: "date-range-picker__error", "{err}" }
            }
        }
    }
}
