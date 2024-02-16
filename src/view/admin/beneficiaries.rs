use crate::model::beneficiaries::beneficiary::{Beneficiaries, Beneficiary};
use crate::model::stats::stats::Stats;
use crate::model::users::user::User;
use crate::view::nav::navigation::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use crate::view::table::beneficiaries_table::BeneficiariesTable;

#[component]
pub fn Beneficiaries(cx: Scope) -> Element {
    let user = use_shared_state::<User>(cx).unwrap();
    let use_beneficiary = use_state(cx, Beneficiary::default);
    let use_beneficiaries = use_shared_state::<Beneficiaries>(cx).unwrap();
    let is_clicked = use_state(cx, || false);
    let navigator = use_navigator(cx);

    load_stats(cx, user, use_shared_state::<Stats>(cx).unwrap());

    let _new = use_future(cx, (user, is_clicked), |(user, click)| {
        let beneficiary = use_beneficiary.clone();
        let beneficiaries = use_beneficiaries.clone();
        let navigator = navigator.clone();
        async move {
            if !click {
                return;
            }
            let bene = Beneficiary::new(user.clone()).await;
            if let Ok(bene) = bene {
                beneficiary.set(bene.clone());
                beneficiaries.with_mut(|benes| benes.beneficiaries.push(bene.clone()));
                navigator.push(Route::BeneficiaryPage { id: bene.Id });
            }
        }
    });
    render! {
        div {
            class: "home-container",
             div{
                class: "beneficiaries-container",
                BeneficiariesTable{},
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

fn load_stats(cx: Scope, user: &UseSharedState<User>, stats: &UseSharedState<Stats>) {
    use_future(cx, user, |user| {
        let stats = stats.clone();
        async move {
            if stats.read().is_empty() {
                let res = Stats::fetch(user).await;
                stats.with_mut(|stats| *stats = res);
            }
        }
    });
}
