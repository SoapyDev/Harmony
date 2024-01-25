use crate::model::beneficiaries::beneficiary::{Beneficiaries, Beneficiary};
use crate::model::beneficiaries::details::{Detail, Presence};
use crate::model::stats::statistiques::city::Cities;
use crate::model::stats::statistiques::family_situation::FamilySituations;
use crate::model::stats::statistiques::income::Incomes;
use crate::model::stats::statistiques::language::Languages;
use crate::model::stats::statistiques::origin::Origins;
use crate::model::stats::statistiques::sexe::Sexes;
use crate::model::stats::statistiques::study::Studies;
use crate::model::stats::traits::Filterable;
use crate::model::users::user::User;
use crate::view::inputs::calendar::{get_weeks, CalendarInputMut, Day};
use crate::view::inputs::dates::DateMut;
use crate::view::inputs::numbers::NumberInputMut;
use crate::view::inputs::selectables::{get_selected_months, get_selected_years, SelectInputMut};
use crate::view::inputs::switches::SwitchInputMut;
use crate::view::inputs::textareas::TextAreaMut;
use crate::view::inputs::texts::TextInputMut;
use chrono::{Datelike, Local};
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;

#[component]
pub fn BeneficiaryPage(cx: Scoped, id: i32) -> Element {
    let user = use_shared_state::<User>(cx).unwrap();
    let beneficiaries = use_shared_state::<Beneficiaries>(cx).unwrap();
    let beneficiary = use_ref(cx, || {
        beneficiaries
            .read()
            .find(id)
            .unwrap_or(&Beneficiary::default())
            .clone()
    });
    let navigator = use_navigator(cx);
    let use_month = use_state(cx, || Local::now().month());
    let use_year = use_state(cx, || Local::now().year());
    let use_day = use_state(cx, || Local::now().day());
    let use_details = use_ref(cx, || Detail::default());

    let update_beneficiary = use_future(cx, (), |_| {
        let use_beneficiary = beneficiary.read().clone();
        let user = user.clone();
        let use_beneficiaries = beneficiaries.clone();
        async move {
            if use_beneficiary.update(user).await {
                use_beneficiaries.with_mut(|val| val.update(&use_beneficiary));
            }
        }
    });

    let add_date = use_future(cx, (), |_| {
        let user = user.clone();
        let use_details = use_details.clone();
        let month = use_month.clone();
        let year = use_year.clone();
        let day = use_day.clone();
        let use_beneficiary = beneficiary.clone();
        let use_beneficiaries = beneficiaries.clone();
        async move {
            let date = format!("{}-{}-{}", year.get(), month.get(), day);
            let presence = Presence {
                BeneficiaryId: use_beneficiary.read().Id,
                Date: date.clone(),
            };
            if Detail::push(user, &use_details, presence.clone()).await {
                use_details.with_mut(|val| val.presences.push(presence));
                use_beneficiary.with_mut(|val| { val.set_last_presence(date) });
                use_beneficiaries.with_mut(|val| val.update(&use_beneficiary.read()));
            }
        }
    });

    let remove_date = use_future(cx, (), |_| {
        let user = user.clone();
        let use_beneficiary = beneficiary.clone();
        let use_details = use_details.clone();
        let month = use_month.clone();
        let year = use_year.clone();
        let day = use_day.clone();
        let use_beneficiaries = beneficiaries.clone();
        async move {
            let date = format!("{}-{}-{}", year.get(), month.get(), day);
            let presence = Presence {
                BeneficiaryId: use_beneficiary.read().Id,
                Date: date.clone(),
            };

            if Detail::pop(user, &use_details, presence).await {
                use_details.with_mut(|val| val.presences.retain(|p| p.Date != date));
                use_beneficiary.with_mut(|val| { val.set_last_presence(date) });
                use_beneficiaries.with_mut(|val| val.update(&use_beneficiary.read()));
            }
        }
    });
    render! {
        style{include_str!("../../assets/style/beneficiary_page.css")}
        header{
            class: "header",
            button{
                class: "back-button",
                onclick: move |_| {
                    navigator.go_back();
                },
                img{src: "../../assets/icon/return.svg"}
            }
        }
        div{
            class: "beneficiary-container",
            form{
                class: "public-form form",
                div{
                    class:"row",
                    TextInputMut{
                        on_input: move |evt: FormEvent| beneficiary.with_mut(|val| val.FirstName = evt.value.to_string()),
                        on_change: move |_: FormEvent| {update_beneficiary.restart()},
                        class: "input",
                        label: "Prénom",
                        value: beneficiary.read().FirstName.to_string(),
                    },
                    TextInputMut{
                        on_input: move |evt: FormEvent| beneficiary.with_mut(|val| val.LastName = evt.value.to_string()),
                        on_change: move |_: FormEvent| {update_beneficiary.restart()},
                        class: "input",
                        label: "Nom",
                        value: beneficiary.read().LastName.to_string(),
                    },
                    DateMut{
                        on_change: move |evt: FormEvent| {
                            beneficiary.with_mut(|val| val.set_birth(&evt.value));
                        },
                        class: "input",
                        label: "Date de naissance",
                        value: beneficiary.read().get_birth(),
                    },
                }
                div{
                    class:"row",
                    SelectInputMut{
                        on_change: move |evt: FormEvent| {
                            beneficiary.with_mut(|val| val.Language = evt.value.to_string());
                            update_beneficiary.restart();
                        },
                        value: beneficiary.read().Language.to_string(),
                        values: Languages::get_selectable_values(),
                        list: Languages::get_labels(),
                        is_multiple: false ,
                        label: "Langue",
                        class: "select",
                    },
                    SelectInputMut{
                        on_change: move |evt: FormEvent| {
                            beneficiary.with_mut(|val| val.Origin = evt.value.to_string());
                            update_beneficiary.restart();
                        },
                        value: beneficiary.read().Origin.to_string(),
                        values: Origins::get_selectable_values(),
                        list: Origins::get_labels(),
                        is_multiple: false ,
                        label: "Provenance",
                        class: "select",
                    },
                    SelectInputMut{
                        on_change: move |evt: FormEvent| {
                            beneficiary.with_mut(|val| val.Sexe = evt.value.to_string());
                            update_beneficiary.restart();
                        },
                        value: beneficiary.read().Sexe.to_string(),
                        values: Sexes::get_selectable_values(),
                        list: Sexes::get_labels(),
                        is_multiple: false ,
                        label: "Sexe",
                        class: "select",
                    },
                },
                div{
                    class: "row",
                    NumberInputMut{
                        on_change: move |evt : FormEvent| {
                            beneficiary.with_mut(move |val| val.Adult = evt.value.parse::<u8>().unwrap_or_default());
                            update_beneficiary.restart();
                        },
                        value:beneficiary.read().Adult.to_string(),
                        label: "Adultes",
                        class: "adultes",
                        min: 0,
                        max: 50,
                    },
                    NumberInputMut{
                        on_change: move |evt: FormEvent|{
                            beneficiary.with_mut(move|val| val.Kid = evt.value.parse::<u8>().unwrap_or_default());
                            update_beneficiary.restart();
                        },
                        value: beneficiary.read().Kid.to_string(),
                        label: "Enfants",
                        class: "enfants",
                        min: 0,
                        max: 50,
                    }
                    TextInputMut{
                        on_input: move |evt: FormEvent| beneficiary.with_mut(|val| val.Phone = evt.value.to_string()),
                        on_change: move |_: FormEvent| {
                            beneficiary.with_mut(|val| val.set_phone());
                            update_beneficiary.restart();
                        },
                        value: beneficiary.read().Phone.to_string(),
                        label: "Téléphone",
                        class: "phone"
                    },
                }
                div{
                    class:"row",
                    div{
                        class: "col",
                        label{ "Allergies" },
                        div{
                            class: "select multiple",
                            select{
                                multiple: true,
                                for allergy in use_details.read().allergies.clone(){
                                    option{
                                        key:"{allergy}",
                                        "{allergy}",
                                    }
                                }
                            }
                        }
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
                        class: "col",
                        div{
                            SwitchInputMut{
                                on_change: move |evt: FormEvent| {
                                    beneficiary.with_mut(move |val| val.IsActive = evt.value.parse::<bool>().unwrap_or_default());
                                    update_beneficiary.restart();
                                },
                                value: beneficiary.read().IsActive,
                                label: "Actif",
                            },
                        }
                    }
                }
                div{
                    class: "row",
                        TextAreaMut{
                            on_input: move |evt: FormEvent| beneficiary.with_mut(|val| val.set_general_note(&evt.value)),
                            on_change: move |_: FormEvent| {update_beneficiary.restart()},
                            value: beneficiary.read().GeneralNote.to_string(),
                            label: "Commentaires généraux",
                            class: "full-width",
                        },
                }
            }
            form{
                class: "private-form form",
                 div{
                    class:"row",
                    TextInputMut{
                        on_input: move |evt: FormEvent| beneficiary.with_mut(|val| val.Address = evt.value.to_string()),
                        on_change: move |_: FormEvent| {update_beneficiary.restart()},
                        value: beneficiary.read().Address.to_string(),
                        class: "input",
                        label: "Address",
                    },
                    SelectInputMut{
                        on_change: move |evt: FormEvent| {
                            beneficiary.with_mut(|val| val.City = evt.value.to_string());
                            update_beneficiary.restart();
                        },
                        value: beneficiary.read().City.to_string(),
                        values: Cities::get_selectable_values(),
                        list: Cities::get_labels(),
                        class: "select",
                        label: "Ville",
                        is_multiple: false ,
                    },
                    TextInputMut{
                        on_input: move |evt:FormEvent| beneficiary.with_mut(|val| val.set_postal_code(&evt.value)),
                        on_change: move |_: FormEvent|{update_beneficiary.restart()},
                        value: beneficiary.read().PostalCode.to_string(),
                        class: "input",
                        label: "Code Postal",
                    },
                }
                 div{
                    class:"row",
                    SelectInputMut{
                        on_change: move |evt: FormEvent| {
                            beneficiary.with_mut(|val| val.Study = evt.value.to_string());
                            update_beneficiary.restart();
                        },
                        value: beneficiary.read().Study.to_string(),
                        values: Studies::get_selectable_values(),
                        list: Studies::get_labels(),
                        class: "select",
                        label: "Étude",
                        is_multiple: false ,
                    },
                    SelectInputMut{
                        on_change: move |evt: FormEvent| {
                            beneficiary.with_mut(|val| val.Income = evt.value.to_string());
                            update_beneficiary.restart();
                        },
                        value: beneficiary.read().Income.to_string(),
                        values: Incomes::get_selectable_values(),
                        list: Incomes::get_labels(),
                        class: "select",
                        label: "Salaire",
                        is_multiple: false ,
                    },
                    SwitchInputMut{
                        on_change: move |_: FormEvent| {
                            beneficiary.with_mut(|val| val.IsEmployed = !val.IsEmployed);
                            update_beneficiary.restart();
                        },
                        value: beneficiary.read().IsEmployed,
                        label: "Emploi",
                    },
                }
                div{
                    class:"row",
                    SelectInputMut{
                        on_change: move |evt: FormEvent| {
                            beneficiary.with_mut(|val| val.FamilySituation = evt.value.to_string());
                            update_beneficiary.restart();
                        },
                        value: beneficiary.read().FamilySituation.to_string(),
                        values: FamilySituations::get_selectable_values(),
                        list: FamilySituations::get_labels(),
                        class: "select",
                        label: "Situation familliale",
                        is_multiple: false ,
                    },
                    div{
                       class: "col",
                        div{
                            class: "span-2",
                            SwitchInputMut{
                                on_change: move |evt: FormEvent| {
                                    beneficiary.with_mut(move |val| val.IsSdf = evt.value.parse::<bool>().unwrap_or_default());
                                    update_beneficiary.restart();
                                },
                                value: beneficiary.read().IsSdf,
                                label: "Sans domicile",
                            },
                        }
                    }
                }
                div{
                    class: "row",
                    TextAreaMut{
                        on_input: move |evt: FormEvent| beneficiary.with_mut(|val| val.Situation = evt.value.to_string()),
                        on_change: move |_: FormEvent|{update_beneficiary.restart()},
                        value: beneficiary.read().Situation.to_string(),
                        label: "Situation",
                        class: "full-width",
                    }
                }
                div{
                    class: "row",
                    TextAreaMut{
                        on_input: move |evt: FormEvent| beneficiary.with_mut(|val| val.AdminNote = evt.value.to_string()),
                        on_change: move |_: FormEvent| {update_beneficiary.restart()},
                        value: beneficiary.read().AdminNote.to_string(),
                        label: "Note à l'administration",
                        class: "",
                    }
                    TextAreaMut{
                        on_input: move |evt: FormEvent| beneficiary.with_mut(|val| val.TsNote = evt.value.to_string()),
                        on_change: move |_: FormEvent| {update_beneficiary.restart()},
                        value: beneficiary.read().TsNote.to_string(),
                        label: "Note privée",
                        class: "span-2",
                    }
                }
            }
        }
    }
}
