use crate::model::beneficiaries::beneficiary::{Beneficiaries, Beneficiary};
use crate::model::beneficiaries::details::Detail;
use crate::model::users::user::User;
use dioxus::prelude::*;

#[component]
pub fn BeneficiariesTable(cx: Scope) -> Element {
    let user = use_shared_state::<User>(cx).unwrap();
    let use_beneficiaries = use_shared_state::<Beneficiaries>(cx).unwrap();
    let use_beneficiary = use_ref(cx, Beneficiary::default);
    let mut selected = "";
    let use_search = use_state(cx, String::new);
    let beneficiaries = use_beneficiaries.read().filter(use_search);
    let use_id = use_state(cx, || 100);

    render! {
        input{
            class: "search-input",
            r#type: "text",
            placeholder: "Rechercher...",
            class: "search-input",
            oninput: move |event| {
                use_search.set(event.value.to_lowercase().clone());
                search(cx, user, use_search, use_beneficiaries);
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
                    if use_beneficiary.read().Id == beneficiary.Id {
                        selected = "selected";
                    }else{
                        selected = "";
                    }
                    tr{
                        ondblclick: move |_| {

                        },
                        key: "{beneficiary.get_id()}",
                        class: "{selected}",
                        td{"{beneficiary.get_full_name()}"},
                        td{"{beneficiary.get_birth()}"},
                        td{"{beneficiary.Adult}"},
                        td{"{beneficiary.Kid}"},
                        td{"{beneficiary.has_allergies()}"},
                        td{"{beneficiary.has_general_note()}"},
                    }
                },
            },
        },
    }
}

fn search(
    cx: Scope,
    user: &UseSharedState<User>,
    use_search: &UseState<String>,
    use_beneficiaries: &UseSharedState<Beneficiaries>,
) {
    use_future(cx, (user, use_search), |(user, search)| {
        let use_beneficiaries = use_beneficiaries.clone();
        async move {
            if search.get().is_empty() || search.get().len() < 3 {
                return;
            }
            let beneficiaries = Beneficiaries::search_beneficiaries(user, search.get()).await;
            use_beneficiaries.with_mut(|benes| benes.insert_beneficiaries(beneficiaries));
        }
    });
}
