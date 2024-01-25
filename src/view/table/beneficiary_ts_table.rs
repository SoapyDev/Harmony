use crate::model::beneficiaries::beneficiary::Beneficiaries;
use crate::view::nav::navigation::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;

#[component]
pub fn TsTable(cx: Scope) -> Element {
    let beneficiaries = use_shared_state::<Beneficiaries>(cx).unwrap();
    let use_search = use_state(cx, String::new);
    let filtered_beneficiaries = beneficiaries.read().filter(use_search);

    render! {
        style{
            include_str!("../../assets/style/ts_table.css"),
        }
        input{
            class: "search-input",
            r#type: "text",
            placeholder: "Search",
            class: "search-input",
            oninput: move |event| {
                use_search.set(event.value.to_lowercase().clone());
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
                    th {"Téléphone"},
                    th {"Note"},
                },
            },
            tbody{
                display: "block",
                for beneficiary in filtered_beneficiaries.beneficiaries {
                    tr{
                        ondblclick: move |_| {
                            let id = beneficiary.Id;
                            use_navigator(cx).push(Route::BeneficiaryPage {
                                id,
                            });
                        },
                        key: "{&beneficiary.Id}",
                        td{"{&beneficiary.get_full_name()"},
                        td{"{&beneficiary.get_birth()}"},
                        td{"{&beneficiary.Adult}"},
                        td{"{&beneficiary.Kid}"},
                        td{"{&beneficiary.Phone" },
                        td{"{&beneficiary.HasGeneralNote}"},
                    }
                },
            },
        },
    }
}
