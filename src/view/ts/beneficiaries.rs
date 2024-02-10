use crate::model::beneficiaries::beneficiary::{Beneficiaries, Beneficiary};
use crate::model::users::user::User;
use crate::view::nav::navigation::Route;
use crate::view::table::beneficiary_ts_table::TsTable;
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;

#[component]
pub fn Beneficiaries(cx: Scope) -> Element {
    let navigator = use_navigator(cx);
    let user = use_shared_state::<User>(cx).unwrap();
    let beneficiaries = use_shared_state::<Beneficiaries>(cx).unwrap();
    let bene = use_state(cx, Beneficiary::default);
    let is_clicked = use_state(cx, || false);

    let _new = use_future(cx, (user, is_clicked), |(user, click)|{
        let beneficiary = bene.clone();
        let beneficiaries = beneficiaries.clone();
        async move{
            if !click {
                return;
            }
            let bene = Beneficiary::new(user.clone()).await;
            if let Ok(bene) = bene {
                beneficiary.set(bene.clone());
                beneficiaries.with_mut(|benes| benes.beneficiaries.push(bene));
            }
        }
    });

    use_future(cx, beneficiaries, |benes| {
        let navigator = navigator.clone();
        let beneficiary = bene.clone();
        let click = is_clicked.clone();
        async move {
            if !click.get() {
                return;
            }
            navigator.push(Route::BeneficiaryPage {id: beneficiary.Id});
        }
    });
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
                            is_clicked.set(true);
                        },
                        "Ajouter"
                    },
                },
            }
        }
    }
}
