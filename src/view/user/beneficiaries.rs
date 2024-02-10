use crate::model::beneficiaries::beneficiary::{Beneficiaries, Beneficiary};
use crate::model::beneficiaries::details::{Detail, Presence};
use crate::model::stats::statistiques::language::Languages;
use crate::model::stats::traits::Filterable;
use crate::model::users::user::User;
use crate::view::inputs::calendar::{get_weeks, CalendarInputMut, Day};
use crate::view::inputs::dates::Date;
use crate::view::inputs::numbers::NumberInput;
use crate::view::inputs::selectables::{get_selected_months, get_selected_years, SelectInput};
use crate::view::inputs::switches::SwitchInput;
use crate::view::inputs::textareas::TextArea;
use crate::view::inputs::texts::TextInput;
use crate::view::table::beneficiary_table::BeneficiariesTable;
use chrono::{Datelike, Local};
use dioxus::prelude::*;

#[component]
pub fn Beneficiaries(cx: Scope) -> Element {
    let use_beneficiaries = use_shared_state::<Beneficiaries>(cx).unwrap();
    let use_beneficiary = use_ref(cx, Beneficiary::default);
    let use_details = use_ref(cx, Detail::default);
    let use_month = use_state(cx, || Local::now().month());
    let use_year = use_state(cx, || Local::now().year());
    let use_day = use_state(cx, || Local::now().day());
    let user = use_shared_state::<User>(cx).unwrap();

    render! {
        div {
            class: "home-container",
            div{
                class: "beneficiaries-container",
                BeneficiariesTable{},
            }
        }
    }
}
