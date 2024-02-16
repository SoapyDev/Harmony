use crate::model::beneficiaries::beneficiary::Beneficiaries;
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use crate::model::users::user::User;

#[component]
pub fn BeneficiariesTable(cx: Scope) -> Element {
    let beneficiaries = use_shared_state::<Beneficiaries>(cx).unwrap();
    let use_search = use_state(cx, String::new);
    let filtered_beneficiaries = beneficiaries.read().filter(use_search);
    let user = use_shared_state::<User>(cx).unwrap();
    let use_id = use_state(cx, || 0);
    let use_navigator = use_navigator(cx);

    use_future(cx, use_id, |id| {
        let use_navigator = use_navigator.clone();
        async move {
            if *id.get() > 99 {
                use_navigator.push(format!("/beneficiary/{}", id.get()));
            }
        }
    });

    render! {
        input{
            class: "search-input",
            r#type: "text",
            placeholder: "Rechercher...",
            class: "search-input",
            oninput: move |event| {
                use_search.set(event.value.to_lowercase().clone());
                search(cx, user, use_search, beneficiaries);
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
                    th {"Dernière présence"},
                    th {"Courriel"},
                    th {"Téléphone"},
                    th {"Présent"},
                },
            },
            tbody{
                display: "block",
                for beneficiary in filtered_beneficiaries.beneficiaries {
                    tr{
                        ondblclick: move |_| {
                            use_id.set(beneficiary.Id);
                        },
                        key: "{beneficiary.Id}",
                        td{"{beneficiary.get_full_name()}"},
                        td{"{beneficiary.get_birth()}"},
                        td{"{beneficiary.LastPresence}"},
                        td{"{beneficiary.Email}"},
                        td{"{beneficiary.get_phone()}"},
                        td{""},
                    }
                },
            },
        },
    }
}


fn search(cx : Scope, user : &UseSharedState<User>, use_search: &UseState<String>, use_beneficiaries: &UseSharedState<Beneficiaries>){
    use_future(cx, (user,use_search), |(user, search)| {
        let use_beneficiaries = use_beneficiaries.clone();
        async move {
            if search.get().is_empty() || search.get().len() < 3 {
                return;
            }
            let beneficiaries = Beneficiaries::search_beneficiaries(user, search.get()).await;
            if beneficiaries.beneficiaries.is_empty() {
                return;
            }
            use_beneficiaries.with_mut(|benes| benes.insert_beneficiaries(beneficiaries));
        }
    });

}

