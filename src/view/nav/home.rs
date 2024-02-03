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
    if user.read().session.is_empty() {
        let navigator = use_navigator(cx);
        navigator.go_back();
    }
    load_beneficiaries(cx, user.clone(), beneficiaries);
    load_stats(cx, user.clone(), stats.clone());
    render! {
        style{include_str!("../../assets/style/home.css")},
        match user.read().role.as_str() {
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
    user: UseSharedState<User>,
    use_beneficiaries: &UseSharedState<Beneficiaries>,
) {
    let _ = use_future(cx, (), |_| {
        let use_beneficiaries = use_beneficiaries.clone();
        let user = user.clone();
        async move {
            if use_beneficiaries.read().beneficiaries.is_empty() {
                let benes = Beneficiaries::get_beneficiaries(user).await;
                use_beneficiaries.with_mut(|val| val.beneficiaries = benes.beneficiaries);
            }
        }
    });
}

fn load_stats(cx: Scope, user: UseSharedState<User>, stats: UseSharedState<Stats>) {
    let _ = use_future(cx, (), |_| {
        let user = user.clone();
        let stats = stats.clone();
        async move {
            if stats.read().is_empty() && user.read().role.eq("Admin") || user.read().role.eq("Dev")
            {
                let res = Stats::fetch(user).await;
                stats.with_mut(|stats| *stats = res);
            }
        }
    });
}
