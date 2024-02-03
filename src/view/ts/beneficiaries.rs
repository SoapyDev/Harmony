use crate::model::beneficiaries::beneficiary::{Beneficiaries, Beneficiary};
use crate::model::users::user::User;
use crate::view::nav::navigation::Route;
use crate::view::table::beneficiary_ts_table::TsTable;
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;

#[component]
pub fn Home(cx: Scope) -> Element {
    let navigator = use_navigator(cx);
    let user = use_shared_state::<User>(cx).unwrap();
    let beneficiaries = use_shared_state::<Beneficiaries>(cx).unwrap();

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
                            let bene = Beneficiary::new(cx, user.clone());
                            let id = bene.Id;
                            beneficiaries.with_mut(|benes| benes.beneficiaries.push(bene));
                            navigator.push(Route::BeneficiaryPage{ id });
                        },
                        "Ajouter"
                    },
                },
            }
        }
    }
}
