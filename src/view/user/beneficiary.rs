use crate::model::beneficiaries::beneficiary::{Beneficiaries, Beneficiary};
use crate::model::beneficiaries::details::{Detail, Presence};
use crate::model::stats::statistiques::language::Languages;
use crate::model::stats::traits::Filterable;
use crate::model::users::user::User;
use crate::view::inputs::calendar::{get_weeks, CalendarInputMut, Day};
use crate::view::inputs::dates::Date;
use crate::view::inputs::selectables::{AllergiesInput, get_selected_months, get_selected_years, SelectInput};
use chrono::{Datelike, Local};
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use crate::view::inputs::numbers::NumberInput;
use crate::view::inputs::switches::SwitchInput;
use crate::view::inputs::textareas::TextArea;
use crate::view::inputs::texts::TextInputMut;

#[component]
pub fn Beneficiary(cx: Scope, id: i32) -> Element {
    let navigator = use_navigator(cx);
    let use_beneficiaries = use_shared_state::<Beneficiaries>(cx).unwrap();
    let use_beneficiary = use_ref(cx, Beneficiary::default);
    let use_month = use_state(cx, || Local::now().month());
    let use_year = use_state(cx, || Local::now().year());
    let use_day = use_state(cx, || Local::now().day());
    let user = use_shared_state::<User>(cx).unwrap();

    let use_details = use_ref(cx, Detail::default);
    get_details(
        cx,
        user,
        use_beneficiary,
        use_beneficiaries,
        use_details,
        id,
    );


    let use_change = use_state(cx, || false);

    let update = use_future(cx, use_change, |change| {
        let user = user.clone();
        let beneficiary = use_beneficiary.clone();
        let beneficiaries = use_beneficiaries.clone();
        async move {
            if !change.get() {
                return;
            }
            let bene = beneficiary.read().clone();
            if bene.update(user).await {
                let id = bene.Id;
                beneficiaries.with_mut(|val| val.update(&beneficiary.read()));
            }
            change.set(false);
        }
    });

    render! {
        header{
            class: "header",
            button{
                class: "back-button",
                onclick: move |_| {
                    navigator.go_back();
                },
                img{src: "./icon/return.svg"}
            }
        }
        div{
            class: "beneficiary-container",
            form{
                class: "public-form form",
                h2{"Informations personnelles"},
                div{
                    class: "row",
                    TextInputMut{
                        on_input: move |evt: FormEvent| use_beneficiary.with_mut(move |val| val.FirstName = evt.value.to_string()),
                        on_change: move |_ : FormEvent| {use_change.set(true)},
                        value:  use_beneficiary.read().FirstName.to_string(),
                        label: "Prénom",
                        class: "input",
                    },
                    TextInputMut{
                        on_input: move |evt: FormEvent| use_beneficiary.with_mut(move |val| val.LastName = evt.value.to_string()),
                        on_change: move |_ : FormEvent| {use_change.set(true)},
                        value: use_beneficiary.read().LastName.to_string(),
                        label: "Nom",
                        class: "input"
                    },
                    Date{
                        value: use_beneficiary.read().get_birth(),
                        placeholder: "Date de naissance",
                        class: "input"
                    },
                }
                div{
                    class: "row",
                    SelectInput{
                        value: use_beneficiary.read().Language.to_string(),
                        values: Languages::get_selectable_values(),
                        list: Languages::get_labels(),
                        is_multiple: false ,
                        label: "Langue",
                        class: "select",
                    },
                     NumberInput{
                        value:use_beneficiary.read().Adult.to_string(),
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
                h2{"Informations complémentaires"},
                div{
                    class: "row",
                        AllergiesInput{
                            detail: use_details.clone(),
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
                    SwitchInput{
                        value: use_beneficiary.read().IsActive,
                        label: "Actif"
                    },
                }
                div{
                    class: "row",
                    TextArea{
                        value: "".to_string(),
                        label: "Notes",
                        class: "full-width",
                    },
                },
            }
        }
    }

}
fn add_date(
    cx: Scope<BeneficiaryProps>,
    user: &UseSharedState<User>,
    beneficiary: &UseRef<Beneficiary>,
    beneficiaries: &UseSharedState<Beneficiaries>,
    use_month: &UseState<u32>,
    use_year: &UseState<i32>,
    use_day: &UseState<u32>,
    use_details: &UseRef<Detail>,
) {
    use_future(
        cx,
        (user, beneficiary, beneficiaries, use_details),
        |(user, beneficiary, beneficiaries, details)| {
            let date = format!("{}-{}-{}", use_year.get(), use_month.get(), use_day.get());
            let presence = Presence {
                BeneficiaryId: beneficiary.read().Id,
                Date: date.clone(),
            };

            async move {
                if Detail::push_presence(user, &details, presence.clone()).await {
                    details.with_mut(|val| val.presences.push(presence));
                    beneficiary.with_mut(|val| val.set_last_presence(date));
                    beneficiaries.with_mut(|val| val.update(&beneficiary.read()));
                }
            }
        },
    );
}
fn remove_date(
    cx: Scope<BeneficiaryProps>,
    user: &UseSharedState<User>,
    beneficiary: &UseRef<Beneficiary>,
    beneficiaries: &UseSharedState<Beneficiaries>,
    use_month: &UseState<u32>,
    use_year: &UseState<i32>,
    use_day: &UseState<u32>,
    use_details: &UseRef<Detail>,
) {
    use_future(
        cx,
        (
            user,
            beneficiary,
            beneficiaries,
            use_month,
            use_year,
            use_day,
            use_details,
        ),
        |(user, beneficiary, beneficiaries, month, year, day, details)| {
            let date = format!("{}-{}-{}", year.get(), month.get(), day);
            let presence = Presence {
                BeneficiaryId: beneficiary.read().Id,
                Date: date.clone(),
            };
            async move {
                if Detail::pop_presence(user, &details, presence).await {
                    details.with_mut(|val| val.presences.retain(|p| p.Date != date));
                    beneficiary.with_mut(|val| val.set_last_presence(date));
                    beneficiaries.with_mut(|val| val.update(&beneficiary.read()));
                }
            }
        },
    );
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
