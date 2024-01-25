use crate::model::beneficiaries::beneficiary::{Beneficiaries, Beneficiary};
use crate::model::beneficiaries::details::Detail;
use crate::model::users::user::User;
use dioxus::prelude::*;

#[derive(Props)]
pub struct TableClickEvent<'a> {
    use_beneficiary: &'a UseRef<Beneficiary>,
    use_details: &'a UseRef<Detail>,
}

#[component]
pub fn BeneficiariesTable<'a>(cx: Scope<'a, TableClickEvent<'a>>) -> Element<'a> {
    let user = use_shared_state::<User>(cx).unwrap();
    let use_beneficiaries = use_shared_state::<Beneficiaries>(cx).unwrap();
    let mut selected = "";
    let use_search = use_state(cx, String::new);
    let beneficiaries = use_beneficiaries.read().filter(use_search);
    let use_id = use_state(cx, || 0);

    let searching = use_future(cx, (), |_| {
        let user = user.clone();
        let use_beneficiaries = use_beneficiaries.clone();
        let use_search = use_search.clone();
        async move {
            let beneficiaries = Beneficiaries::search_beneficiaries(user, use_search.get()).await;
            use_beneficiaries.with_mut(|benes| benes.insert_beneficiaries(beneficiaries));
        }
    });

    let details = use_future(cx, use_id, |id| {
        let beneficiary = cx.props.use_beneficiary.clone();
        let user = user.clone();
        let details = cx.props.use_details.clone();
        async move {
            let (bene, detail) = Beneficiary::get_beneficiary(user, *id.get()).await;
            beneficiary.set(bene.clone());
            details.set(detail.clone());
        }
    });
    render! {
        input{
            class: "search-input",
            r#type: "text",
            placeholder: "Search",
            class: "search-input",
            oninput: move |event| {
                use_search.set(event.value.to_lowercase().clone());
                if use_search.get().len() >= 3  {
                    searching.restart();
                }
            },
        },
        table{
            table_layout: "fixed",
            border_collapse: "collapse",
            display: "block",
            thead {
                display: "block",
                tr {
                    th {"Nom"},
                    th {"Date de naissance"},
                    th {"Adulte"},
                    th {"Enfant"},
                    th {"Allergies"},
                    th {"Notes"},
                },
            },
            tbody{
                display: "block",
                for beneficiary in beneficiaries.beneficiaries {
                    if cx.props.use_beneficiary.read().Id == beneficiary.Id {
                        selected = "selected";
                    }else{
                        selected = "";
                    }
                    tr{
                        onclick: move |_| {
                            use_id.set(beneficiary.Id);
                            details.restart()
                        },
                        key: "{&beneficiary.get_id()}",
                        class: "{selected}",
                        td{"{&beneficiary.get_full_name()}"},
                        td{"{&beneficiary.get_birth()}"},
                        td{"{&beneficiary.Adult}"},
                        td{"{&beneficiary.Kid}"},
                        td{"{&beneficiary.has_allergies()}"},
                        td{"{&beneficiary.has_general_note()}"},
                    }
                },
            },
        },
    }
}
