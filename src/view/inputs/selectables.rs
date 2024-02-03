use crate::model::beneficiaries::details::{Allergy, Detail};
use crate::model::users::user::User;
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

static ALLERGIES: &[&str; 13] = &[
    "Noix",
    "Arachides",
    "Lactose",
    "Gluten",
    "Oeufs",
    "Fruits de mer",
    "Poisson",
    "Ble",
    "Soja",
    "Sesame",
    "Moutarde",
    "Sulfites",
    "Autre",
];

#[component]
pub(crate) fn AllergiesInputMut<'a>(
    cx: Scope,
    beneficiary_id: i32,
    details: &'a UseRef<Detail>,
) -> Element {
    let allergy_list = use_ref(cx, || ALLERGIES.to_vec());
    let selected_allergy = use_state(cx, || "".to_string());
    let user = use_shared_state::<User>(cx).unwrap();
    let use_is_add = use_state(cx, || false);
    let use_is_remove = use_state(cx, || false);

    let _add = use_future(cx, use_is_add, |add| {
        let selected_allergy = selected_allergy.clone();
        let details = details.to_owned().clone();
        let beneficiary_id = *beneficiary_id;
        let user = user.clone();

        async move {
            if *add.get() {
                if let Ok(()) =
                    Detail::add_allergy(beneficiary_id, selected_allergy.get(), user.clone()).await
                {
                    details.with_mut(|detail| {
                        detail.allergies.push(Allergy {
                            BeneficiaryId: beneficiary_id,
                            Allergy: selected_allergy.get().clone(),
                        })
                    });
                }
                add.set(false);
            }
        }
    });

    let _remove = use_future(cx, use_is_remove, |remove| {
        let selected_allergy = selected_allergy.clone();
        let details = details.to_owned().clone();
        let beneficiary_id = *beneficiary_id;
        let user = user.clone();

        async move {
            if *remove.get() {
                if let Ok(()) =
                    Detail::remove_allergy(beneficiary_id, selected_allergy.get(), user.clone())
                        .await
                {
                    details.with_mut(|detail| {
                        detail
                            .allergies
                            .retain(|allergy| &allergy.Allergy != selected_allergy.get());
                    });
                }
                remove.set(false);
            }
        }
    });

    render! {
        div{
            class: "row",
            div{
                class: "col",
                label {"Allergies"},
                select {
                    onchange: |evt| {
                       selected_allergy.set(evt.value.to_string());
                    },
                    class : "form-control",
                    multiple : true,
                    for element in allergy_list.read().iter(){
                        if !details.read().allergies.iter().any(|a| a.Allergy == *element) {
                            render!{
                                option{
                                    selected: "{selected_allergy.get() == *element}",
                                    key: "{element}",
                                    value: "{element}",
                                    "{element}",
                                }
                            }
                        }
                    }
                }
            }
            div{
                class: "col",
                button{
                    onclick: |_|{
                        if details.read().allergies.len() < ALLERGIES.len() {
                            use_is_add.set(true);
                        };
                    },
                    ">>"
                },
                button{
                    onclick: |_|{
                        if  !details.read().allergies.is_empty() {
                            use_is_remove.set(true);
                        };
                    },
                    "<<"
                },
            }
            div{
                class: "col",
                label {"Bénéficiaire"},
                select {
                    onchange: |evt| {
                      selected_allergy.set(evt.value.to_string());
                    },
                    class : "form-control",
                    multiple : true,
                    for element in details.read().allergies.iter(){
                        render!{
                            option{
                                selected: "{selected_allergy.get() == &element.Allergy}",
                                key: "{element.Allergy}",
                                value: "{element.Allergy}",
                                "{element.Allergy}",
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
