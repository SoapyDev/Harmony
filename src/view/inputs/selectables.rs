use chrono::{Datelike, Local};
use dioxus::prelude::*;

#[derive(Props)]
pub struct SelectChangeEvent<'a> {
    on_change: EventHandler<'a, FormEvent>,
    value: String,
    values: Vec<String>,
    list: Vec<String>,
    is_multiple: bool,
    label: &'a str,
    class: &'a str,
}

#[component]
pub fn SelectInput<'a>(
    cx: Scope<'a>,
    value: String,
    values: Vec<String>,
    list: Vec<String>,
    is_multiple: bool,
    label: &'a str,
    class: &'a str,
) -> Element<'a> {
    render! {
        div{
            class: "col",
            label {"{label}"},
            select {
                class : "form-control {class}",
                disabled : "true",
                multiple : "{is_multiple}",
                for (i,element) in values.iter().enumerate(){
                    option{
                        key: "{element}",
                        value: "{element}",
                        selected: "{value}",
                        "{list[i]}",
                    }
                }
            }
        }
    }
}

#[component]
pub fn SelectInputMut<'a>(cx: Scope<'a, SelectChangeEvent<'a>>) -> Element<'a> {
    let use_values = use_state(cx, || cx.props.values.clone());
    let use_list = use_state(cx, || cx.props.list.clone());
    render! {
        div{
            class: "col",
            label {cx.props.label},
            select {
                onchange: |evt| {
                    cx.props.on_change.call(evt);
                },
                class : "form-control {cx.props.class}",
                multiple : "{cx.props.is_multiple}",
                for (i,element) in use_values.iter().enumerate(){
                    if element == &cx.props.value{
                        render!{
                            option{
                                selected: true,
                                key: "{element}",
                                value: "{element}",
                                "{use_list[i]}",
                            }
                        }
                    }else{
                        render!{
                            option{
                                key: "{element}",
                                value: "{element}",
                                "{use_list[i]}",
                            }
                        }
                    }
                }
            }
        }
    }
}

static MONTHS: [&str; 12] = [
    "Janvier",
    "Février",
    "Mars",
    "Avril",
    "Mai",
    "Juin",
    "Juillet",
    "Aout",
    "Septembre",
    "Octobre",
    "Novembre",
    "Décembre",
];
pub fn get_selected_months<'a, T>(
    cx: &'a Scoped<'a, T>,
    use_month: &'a UseState<u32>,
) -> Element<'a> {
    render! {
        select{
            onchange: move |evt| use_month.set(evt.value.parse::<u32>().unwrap()),
            class: "select",
            for month in 1..=12{
                option{
                    value: "{month}",
                    selected: "{month == *use_month.get()}",
                    "{MONTHS[month as usize - 1]}",
                }
            }
        }
    }
}

pub fn get_selected_years<'a, T>(
    cx: &'a Scoped<'a, T>,
    use_year: &'a UseState<i32>,
) -> Element<'a> {
    let current_year = Local::now().year();
    render! {
        select{
            onchange: move |evt| use_year.set(evt.value.parse::<i32>().unwrap()),
            class: "select",
            for year in (2000..=current_year).rev(){
                option{
                    selected: "{*use_year.get() == year}",
                    "{year}",
                }
            }
        }
    }
}
