use crate::model::beneficiaries::beneficiary::{Beneficiaries, Beneficiary};
use crate::model::beneficiaries::details::{Detail, Presence};
use crate::model::stats::statistiques::language::Languages;
use crate::model::stats::statistiques::origin::Origins;
use crate::model::stats::statistiques::sexe::Sexes;
use crate::model::stats::traits::Filterable;
use crate::model::users::user::User;
use crate::view::inputs::calendar::{get_weeks, CalendarInputMut, Day};
use crate::view::inputs::dates::DateMut;
use crate::view::inputs::numbers::NumberInputMut;
use crate::view::inputs::selectables::{
    get_selected_months, get_selected_years, AllergiesInputMut, SelectInputMut,
};
use crate::view::inputs::switches::SwitchInputMut;
use crate::view::inputs::textareas::TextAreaMut;
use crate::view::inputs::texts::TextInputMut;
use crate::view::table::beneficiary_table::BeneficiariesTable;
use chrono::{Datelike, Local, NaiveDate};
use dioxus::prelude::*;
#[component]
pub fn Home(cx: Scope) -> Element {
    let user = use_shared_state::<User>(cx).unwrap();
    let use_beneficiaries = use_shared_state::<Beneficiaries>(cx).unwrap();
    let use_detail = use_ref(cx, Detail::default);
    let use_beneficiary = use_ref(cx, Beneficiary::default);
    let use_month = use_state(cx, || Local::now().month());
    let use_year = use_state(cx, || Local::now().year());
    let use_day = use_state(cx, || Local::now().day());

    let update_beneficiary = use_future(cx, (), |_| {
        let use_beneficiary = use_beneficiary.read().clone();
        let user = user.clone();
        let use_beneficiaries = use_beneficiaries.clone();
        async move {
            if use_beneficiary.update(user).await {
                use_beneficiaries.with_mut(|val| val.update(&use_beneficiary));
            }
        }
    });

    let add_date = use_future(cx, (), |_| {
        let user = user.clone();
        let use_detail = use_detail.clone();
        let month = use_month.clone();
        let year = use_year.clone();
        let day = use_day.clone();
        let use_beneficiary = use_beneficiary.clone();
        let use_beneficiaries = use_beneficiaries.clone();
        async move {
            let date = NaiveDate::from_ymd_opt(*year.get(), *month.get(), *day).unwrap();
            let date = date.format("%Y-%m-%d").to_string();
            let presence = Presence {
                BeneficiaryId: use_beneficiary.read().Id,
                Date: date.clone(),
            };
            if Detail::push_presence(user, &use_detail, presence.clone()).await {
                use_detail.with_mut(|val| val.presences.push(presence));
                use_beneficiary.with_mut(|val| val.set_last_presence(date));
                use_beneficiaries.with_mut(|val| val.update(&use_beneficiary.read()));
            }
        }
    });

    let remove_date = use_future(cx, (), |_| {
        let user = user.clone();
        let use_detail = use_detail.clone();
        let month = use_month.clone();
        let year = use_year.clone();
        let day = use_day.clone();
        let use_beneficiary = use_beneficiary.clone();
        let use_beneficiaries = use_beneficiaries.clone();
        async move {
            let date = NaiveDate::from_ymd_opt(*year.get(), *month.get(), *day).unwrap();
            let date = date.format("%Y-%m-%d").to_string();
            let presence = Presence {
                BeneficiaryId: use_beneficiary.read().Id,
                Date: date.clone(),
            };

            if Detail::pop_presence(user, &use_detail, presence).await {
                use_detail.with_mut(|val| val.presences.retain(|p| p.Date != date));
                use_beneficiary.with_mut(|val| val.set_last_presence(date));
                use_beneficiaries.with_mut(|val| val.update(&use_beneficiary.read()));
            }
        }
    });

    render! {
        div {
            class: "home-container",
            div{
                class: "beneficiaries-container",
                BeneficiariesTable{
                    use_beneficiary: use_beneficiary,
                    use_details: use_detail,
                },
            }
            div{
                class: "beneficiary-container",
                form{
                    div {
                        class: "form-group input-block",
                        div{
                            class: "row",
                            TextInputMut{
                                on_input: move |evt: FormEvent| use_beneficiary.with_mut(move |val| val.FirstName = evt.value.to_string()),
                                on_change: move |_ : FormEvent| {update_beneficiary.restart()},
                                value:  use_beneficiary.read().FirstName.to_string(),
                                label: "Prénom",
                                class: "",
                            },
                            TextInputMut{
                                on_input: move |evt: FormEvent| use_beneficiary.with_mut(move |val| val.LastName = evt.value.to_string()),
                                on_change: move |_ : FormEvent| {update_beneficiary.restart()},
                                value: use_beneficiary.read().LastName.to_string(),
                                label: "Nom",
                                class: "lastname"
                            },
                        }
                        div{
                            class: "row",
                            DateMut{
                                    on_change: move |event : FormEvent| {
                                        use_beneficiary.with_mut(move |val| val.set_birth(&event.value));
                                        update_beneficiary.restart()
                                    },
                                    value: use_beneficiary.read().get_birth(),
                                    label: "Date de naissance",
                                    class: "birthdate",
                            },
                            SelectInputMut{
                                on_change: move |evt: FormEvent| {
                                    use_beneficiary.with_mut(move |val| val.Language = evt.value.to_string());
                                    update_beneficiary.restart()
                                },
                                value: use_beneficiary.read().Language.to_string(),
                                values: Languages::get_selectable_values(),
                                list: Languages::get_labels(),
                                is_multiple: false ,
                                label: "Langue",
                                class: "select",
                            },
                        },
                            div{
                                class: "row",
                                SelectInputMut{
                                    on_change: move |evt: FormEvent| {
                                        use_beneficiary.with_mut(move |val| val.Sexe = evt.value.to_string());
                                        update_beneficiary.restart()
                                    },
                                    value: use_beneficiary.read().Sexe.to_string(),
                                    values: Sexes::get_selectable_values(),
                                    list: Sexes::get_labels(),
                                    is_multiple: false ,
                                    label: "Sexe",
                                    class: "select"
                                },
                                SelectInputMut{
                                    on_change: move |evt: FormEvent| {
                                        use_beneficiary.with_mut(|val| val.Origin = evt.value.to_string());
                                        update_beneficiary.restart()
                                    },
                                    value: use_beneficiary.read().Origin.to_string(),
                                    values: Origins::get_selectable_values(),
                                    list: Origins::get_labels(),
                                    is_multiple: false ,
                                    label: "Origine",
                                    class: "select"
                                },
                            },
                        div{
                            class: "row",
                            div{
                                class: "inputs-row",
                                NumberInputMut{
                                    on_change: move |evt : FormEvent| {
                                        use_beneficiary.with_mut(move|val| val.Adult = evt.value.parse::<u8>().unwrap_or_default());
                                        update_beneficiary.restart()
                                    },
                                    value:use_beneficiary.read().Adult.to_string(),
                                    label: "Adultes",
                                    class: "adultes",
                                    min: 0,
                                    max: 50,
                                },
                                NumberInputMut{
                                    on_change: move |evt: FormEvent|{
                                        use_beneficiary.with_mut(move|val| val.Kid = evt.value.parse::<u8>().unwrap_or_default());
                                        update_beneficiary.restart()
                                    },
                                    value: use_beneficiary.read().Kid.to_string(),
                                    label: "Enfants",
                                    class: "enfants",
                                    min: 0,
                                    max: 50,
                                }
                            },
                            div{
                                class: "inputs-row",
                                TextInputMut{
                                    on_input: move |evt: FormEvent| use_beneficiary.with_mut(|val| val.Phone = evt.value.to_string()),
                                    on_change: move |_| {
                                        use_beneficiary.with_mut(|val| val.set_phone());
                                        update_beneficiary.restart()
                                    },
                                    value: use_beneficiary.read().Phone.to_string(),
                                    label: "Téléphone",
                                    class: "phone"
                                },
                                SwitchInputMut{
                                    on_change: move |evt: FormEvent| {
                                        use_beneficiary.with_mut(move |val| { val.IsActive = evt.value.parse::<bool>().unwrap_or_default()});
                                        update_beneficiary.restart()
                                    },
                                    value: use_beneficiary.read().IsActive,
                                    label: "Actif"
                                },
                            },
                        },
                    },
                    div{
                        class: "form-group allergies",
                        AllergiesInputMut{
                            beneficiary_id: use_beneficiary.read().Id,
                            details: use_detail,
                        },
                    },
                   div{
                       class: "form-group calendar",
                       div{
                           class: "col",
                           label{"Présences"},
                           div{
                               class: "calendar-header",
                               get_selected_months(cx, use_month),
                               get_selected_years(cx, use_year),
                           }
                            CalendarInputMut{
                                on_click: move |val : &Day| {
                                    use_day.set(val.day);
                                    add_date.restart();
                                },
                                on_dbclick: move |val: &Day| {
                                    use_day.set(val.day);
                                    remove_date.restart();
                                },
                                month: get_weeks(*use_year.get(), *use_month.get(), &use_detail.read().presences),
                                current_month: *use_month.get(),
                                current_year: *use_year.get(),
                                current_day: Local::now().day(),
                            },
                       }
                     },
                    div{
                        class: "form-group notes",
                        TextAreaMut{
                            on_input: move |evt: FormEvent| use_beneficiary.with_mut(|val| val.set_general_note(&evt.value)),
                            on_change: move |_: FormEvent|{update_beneficiary.restart()},
                            value: use_beneficiary.read().GeneralNote.to_string(),
                            label: "Notes",
                            class: "",
                        },

                    },
                }
            }

        }
    }
}
