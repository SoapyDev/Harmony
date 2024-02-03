use crate::model::beneficiaries::beneficiary::{Beneficiaries, Beneficiary};
use crate::model::beneficiaries::details::{Detail, Presence};
use crate::model::stats::statistiques::language::Languages;
use crate::model::stats::traits::Filterable;
use crate::model::users::user::User;
use crate::view::inputs::calendar::{get_weeks, CalendarInputMut, Day};
use crate::view::inputs::dates::Date;
use crate::view::inputs::numbers::NumberInput;
use crate::view::inputs::selectables::{get_selected_months, get_selected_years, SelectInput};
use crate::view::inputs::switches::SwitchInput;
use crate::view::inputs::textareas::TextArea;
use crate::view::inputs::texts::TextInput;
use crate::view::table::beneficiary_table::BeneficiariesTable;
use chrono::{Datelike, Local};
use dioxus::prelude::*;

#[component]
pub fn Home(cx: Scope) -> Element {
    use_shared_state_provider(cx, Beneficiaries::new);
    let use_beneficiaries = use_shared_state::<Beneficiaries>(cx).unwrap();
    let use_beneficiary = use_ref(cx, Beneficiary::default);
    let use_details = use_ref(cx, Detail::default);
    let use_month = use_state(cx, || Local::now().month());
    let use_year = use_state(cx, || Local::now().year());
    let use_day = use_state(cx, || Local::now().day());
    let user = use_shared_state::<User>(cx).unwrap();

    let add_date = use_future(cx, (), |_| {
        let user = user.clone();
        let use_details = use_details.clone();
        let month = use_month.clone();
        let year = use_year.clone();
        let day = use_day.clone();
        let use_beneficiary = use_beneficiary.clone();
        let use_beneficiaries = use_beneficiaries.clone();
        async move {
            let date = format!("{}-{}-{}", year.get(), month.get(), day);
            let presence = Presence {
                BeneficiaryId: use_beneficiary.read().Id,
                Date: date.clone(),
            };
            if Detail::push_presence(user, &use_details, presence.clone()).await {
                use_details.with_mut(|val| val.presences.push(presence));
                use_beneficiary.with_mut(|val| val.set_last_presence(date));
                use_beneficiaries.with_mut(|val| val.update(&use_beneficiary.read()));
            }
        }
    });

    let remove_date = use_future(cx, (), |_| {
        let user = user.clone();
        let use_details = use_details.clone();
        let month = use_month.clone();
        let year = use_year.clone();
        let day = use_day.clone();
        let use_beneficiary = use_beneficiary.clone();
        let use_beneficiaries = use_beneficiaries.clone();
        async move {
            let date = format!("{}-{}-{}", year.get(), month.get(), day);
            let presence = Presence {
                BeneficiaryId: use_beneficiary.read().Id,
                Date: date.clone(),
            };

            if Detail::pop_presence(user, &use_details, presence).await {
                use_details.with_mut(|val| val.presences.retain(|p| p.Date != date));
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
                    use_details: use_details,
                },
            }
            div{
                class: "beneficiary-container",
                form{
                    div {
                        class: "form-group input-block",
                        div{
                            class: "row",
                            TextInput{
                                value: use_beneficiary.read().FirstName.to_string(),
                                label: "Prénom",
                                class: "",
                            },
                            TextInput{
                                value: use_beneficiary.read().LastName.to_string(),
                                label: "Nom",
                                class: "lastname"
                            },
                        }
                        div{
                            class: "row",
                            Date{
                              value: use_beneficiary.read().get_birth(),
                              placeholder: "Date de naissance",
                              class:  "birthdate",
                            }
                            SelectInput{
                                value: use_beneficiary.read().Language.to_string(),
                                values: Languages::get_selectable_values(),
                                list: Languages::get_labels(),
                                is_multiple: false ,
                                label: "Langue",
                                class: "select"
                            },
                        },
                        div{
                            class: "row",
                            div{
                                class: "inputs-row",
                                NumberInput{
                                    value: use_beneficiary.read().Adult.to_string(),
                                    label: "Adultes",
                                    class: "adultes",
                                    min: 0,
                                    max: 50,
                                },
                                NumberInput{
                                    value: use_beneficiary.read().Kid.to_string(),
                                    label: "Enfants",
                                    class: "enfants",
                                    min: 0,
                                    max: 50,

                                },
                            },
                            div{
                                class: "inputs-row",
                                SwitchInput{
                                    value: use_beneficiary.read().IsActive,
                                    label: "Actif"
                                },
                            }
                        },
                    },
                    div{
                        class: "form-group allergies",
                        div{
                            class: "col",
                            label{ "Allergies" },
                            div{
                                class: "select multiple",
                                select{
                                    multiple: true,
                                    for allergy in use_details.read().get_allergies(){
                                        option{
                                            "{allergy}",
                                        }
                                    }
                                }
                            }
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
                                month: get_weeks(*use_year.get(), *use_month.get(), &use_details.read().presences),
                                current_month: *use_month.get(),
                                current_year: *use_year.get(),
                                current_day: Local::now().day(),
                            },
                       }
                     },
                    div{
                        class: "form-group notes",
                        TextArea{
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
