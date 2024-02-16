#![allow(non_snake_case)]

use crate::model::beneficiaries::beneficiary::Beneficiaries;
use crate::model::stats::stats::Stats;
use crate::model::users::user::User;
use crate::view::{admin, ts, user};
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;

#[component]
pub fn BeneficiaryPage(cx: Scope, id: i32) -> Element {
    let user = use_shared_state::<User>(cx).unwrap();
    let beneficiaries = use_shared_state::<Beneficiaries>(cx).unwrap();
    let _stats = use_shared_state::<Stats>(cx).unwrap();
    if user.read().Token.is_empty() {
        let navigator = use_navigator(cx);
        navigator.go_back();
    }

    render! {
        style{include_str!("../../assets/style/beneficiary.css")},
        match user.read().Role.as_str() {
            "Admin" | "Dev" => render! {
                admin::beneficiary::Beneficiary{
                    id: *id,
                }
            },
            "TS" => render! {
                ts::beneficiary::Beneficiary{
                    id: *id,
                }
            },
            _ => render!{
                    user::beneficiary::Beneficiary{
                        id: *id,
                    }
                }
            ,
        }
    }
}
