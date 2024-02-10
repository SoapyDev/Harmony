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
use chrono::{Datelike, Local};
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;

#[component]
pub fn Beneficiary(cx: Scoped, id: i32) -> Element {
    let navigator = use_navigator(cx);
    let user = use_shared_state::<User>(cx).unwrap();

    let use_beneficiaries = use_shared_state::<Beneficiaries>(cx).unwrap();
    let use_beneficiary = use_ref(cx, || {
        use_beneficiaries
            .read()
            .find(id)
            .unwrap_or(&Beneficiary::default())
            .clone()
    });

    let use_details = use_ref(cx, Detail::default);
    get_details(
        cx,
        user,
        use_beneficiary,
        use_beneficiaries,
        use_details,
        id,
    );

    let use_month = use_state(cx, || Local::now().month());
    let use_year = use_state(cx, || Local::now().year());
    let use_day = use_state(cx, || Local::now().day());
    render! {
            div{
                class: "beneficiary-container",
                form{
                    div {
                        class: "form-group input-block",
                        div{
                            class: "row",
                            TextInputMut{
                                on_input: move |evt: FormEvent| use_beneficiary.with_mut(move |val| val.FirstName = evt.value.to_string()),
                                on_change: move |_ : FormEvent| {},
                                value:  use_beneficiary.read().FirstName.to_string(),
                                label: "Prénom",
                                class: "",
                            },
                            TextInputMut{
                                on_input: move |evt: FormEvent| use_beneficiary.with_mut(move |val| val.LastName = evt.value.to_string()),
                                on_change: move |_ : FormEvent| {},
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
                                    },
                                    value: use_beneficiary.read().get_birth(),
                                    label: "Date de naissance",
                                    class: "birthdate",
                            },
                            SelectInputMut{
                                on_change: move |evt: FormEvent| {
                                    use_beneficiary.with_mut(move |val| val.Language = evt.value.to_string());
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
                                    },
                                    value: use_beneficiary.read().Phone.to_string(),
                                    label: "Téléphone",
                                    class: "phone"
                                },
                                SwitchInputMut{
                                    on_change: move |evt: FormEvent| {
                                        use_beneficiary.with_mut(move |val| { val.IsActive = evt.value.parse::<bool>().unwrap_or_default()});
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
                            details: use_details,
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
                                    add_date(cx, user, use_beneficiary, use_beneficiaries, use_month, use_year, use_day, use_details);
                                },
                                on_dbclick: move |val: &Day| {
                                    use_day.set(val.day);
                                    remove_date(cx, user, use_beneficiary, use_beneficiaries, use_month, use_year, use_day, use_details);
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
                        TextAreaMut{
                            on_input: move |evt: FormEvent| use_beneficiary.with_mut(|val| val.set_general_note(&evt.value)),
                            on_change: move |_: FormEvent|{},
                            value: use_beneficiary.read().GeneralNote.to_string(),
                            label: "Notes",
                            class: "",
                        },

                    },
                }
            }

    }
}
fn update_beneficiary(
    cx: Scope,
    beneficiary: &UseRef<crate::model::beneficiaries::beneficiary::Beneficiary>,
    user: &UseSharedState<User>,
    beneficiaries: &UseSharedState<Beneficiaries>,
) {
    use_future(cx, user, |user| {
        let beneficiary = beneficiary.clone();
        let beneficiaries = beneficiaries.clone();
        async move {
            let bene = beneficiary.read().clone();
            if bene.update(user).await {
                beneficiaries.with_mut(|val| val.update(&beneficiary.read()));
            }
        }
    });
}

fn add_date(
    cx: Scope<BeneficiaryProps>,
    user: &UseSharedState<User>,
    beneficiary: &UseRef<crate::model::beneficiaries::beneficiary::Beneficiary>,
    beneficiaries: &UseSharedState<Beneficiaries>,
    use_month: &UseState<u32>,
    use_year: &UseState<i32>,
    use_day: &UseState<u32>,
    use_details: &UseRef<Detail>,
) {
    use_future(cx, user, |user| {
        let date = format!("{}-{}-{}", use_year.get(), use_month.get(), use_day.get());
        let presence = Presence {
            BeneficiaryId: beneficiary.read().Id,
            Date: date.clone(),
        };
        let details = use_details.clone();
        let beneficiary = beneficiary.clone();
        let beneficiaries = beneficiaries.clone();

        async move {
            if Detail::push_presence(user, &details, presence.clone()).await {
                details.with_mut(|val| val.presences.push(presence));
                beneficiary.with_mut(|val| val.set_last_presence(date));
                beneficiaries.with_mut(|val| val.update(&beneficiary.read()));
            }
        }
    });
}
fn remove_date(
    cx: Scope<BeneficiaryProps>,
    user: &UseSharedState<User>,
    beneficiary: &UseRef<crate::model::beneficiaries::beneficiary::Beneficiary>,
    beneficiaries: &UseSharedState<Beneficiaries>,
    use_month: &UseState<u32>,
    use_year: &UseState<i32>,
    use_day: &UseState<u32>,
    use_details: &UseRef<Detail>,
) {
    use_future(cx, user, |user| {
        let date = format!("{}-{}-{}", use_year.get(), use_month.get(), use_day);
        let presence = Presence {
            BeneficiaryId: beneficiary.read().Id,
            Date: date.clone(),
        };
        let details = use_details.clone();
        let beneficiary = beneficiary.clone();
        let beneficiaries = beneficiaries.clone();
        async move {
            if Detail::pop_presence(user, &details, presence).await {
                details.with_mut(|val| val.presences.retain(|p| p.Date != date));
                beneficiary.with_mut(|val| val.set_last_presence(date));
                beneficiaries.with_mut(|val| val.update(&beneficiary.read()));
            }
        }
    });
}
fn get_details(
    cx: Scope<BeneficiaryProps>,
    user: &UseSharedState<User>,
    beneficiary: &UseRef<Beneficiary>,
    beneficiaries: &UseSharedState<Beneficiaries>,
    use_details: &UseRef<Detail>,
    id: &i32,
) {
    use_future(cx, (user, id), |(user, id)| {
        let details = use_details.clone();
        let beneficiary = beneficiary.clone();
        let beneficiaries = beneficiaries.clone();

        async move {
            let (bene, detail) = Beneficiary::get_beneficiary(user, id).await;
            beneficiary.set(bene.clone());
            details.set(detail.clone());
            beneficiaries.with_mut(|benes| benes.update(&bene))
        }
    });
}
