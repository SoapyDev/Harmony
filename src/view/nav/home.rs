#![allow(non_snake_case)]

use crate::model::beneficiaries::beneficiary::Beneficiaries;
use crate::model::stats::stats::Stats;
use crate::model::users::user::User;
use crate::view::{admin, ts, user};
use dioxus::prelude::*;
use dioxus_hooks::use_shared_state;
use dioxus_router::hooks::use_navigator;

pub fn Home(cx: Scope) -> Element {
    let user = use_shared_state::<User>(cx).unwrap();
    let beneficiaries = use_shared_state::<Beneficiaries>(cx).unwrap();
    let stats = use_shared_state::<Stats>(cx).unwrap();
    if user.read().Token.is_empty() {
        let navigator = use_navigator(cx);
        navigator.go_back();
    }
    load_beneficiaries(cx, user, beneficiaries);

    render! {
        style{include_str!("../../assets/style/home.css")},
        match user.read().Role.as_str() {
            "Admin" | "Dev" => render! {
                admin::beneficiary::Home{}
            },
            "TS" => render! {
                ts::beneficiaries::Home{}
            },
            _ => render!{
                    user::Home{}
                }
            ,
        }
    }
}
fn load_beneficiaries(
cx: Scope,
user: &UseSharedState<User>,
use_beneficiaries: &UseSharedState<Beneficiaries>,
) {
use_future(cx, (), |_| {
    let beneficiaries = use_beneficiaries.clone();
    let user = user.clone();
    async move {
        if beneficiaries.read().beneficiaries.is_empty() {
            let benes = Beneficiaries::get_beneficiaries(user).await;
            beneficiaries.with_mut(|val| val.beneficiaries = benes.beneficiaries);
        }
    }
});
}


