use crate::model::beneficiaries::beneficiary::Beneficiary;
use crate::view::nav::navigation::Route;
use crate::view::table::beneficiary_ts_table::TsTable;
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;

#[component]
pub fn Home(cx: Scope) -> Element {
    let navigator = use_navigator(cx);

    render! {
        div {
            class: "home-container",
             div{
                class: "beneficiaries-container",
                    TsTable{}
                div{
                    button{
                        class: "add-button",
                        onclick: move |_| {
                            let bene = Beneficiary::default();
                            let id = bene.Id;
                            navigator.push(Route::BeneficiaryPage{ id });
                        },
                        "Ajouter"
                    },
                },
            }
        }
    }
}
