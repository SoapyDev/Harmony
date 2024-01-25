use crate::model::beneficiaries::beneficiary::Beneficiaries;
use crate::model::stats::stats::Stats;
use crate::model::users::user::User;
use crate::view::nav::navigation::Route;
use dioxus::prelude::*;
use dioxus_router::components::Router;

pub(crate) fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, User::new);
    use_shared_state_provider(cx, Beneficiaries::new);
    use_shared_state_provider(cx, Stats::new);

    render! {
            style { include_str!("./assets/style/style.css") },
        body{
            main{
                Router::<Route> {}
            }
        }
    }
}
