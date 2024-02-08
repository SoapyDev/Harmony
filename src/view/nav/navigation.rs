use crate::model::beneficiaries::beneficiary::Beneficiaries;
use crate::model::users::{role::Role, user::User};
use crate::view::admin::{stats::StatsPage, users::Users};
use crate::view::login::Login;
use crate::view::nav::home::Home;
use crate::view::ts::beneficiary::BeneficiaryPage;
use dioxus::prelude::*;
use dioxus_router::components::{Link, Outlet};
use dioxus_router::hooks::use_navigator;
use dioxus_router::prelude::Routable;

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
pub(crate) enum Route {
    #[route("/")]
    Login {},
    #[layout(Nav)]
    #[route("/home")]
    Home {},
    #[route("/home/:id")]
    BeneficiaryPage { id: i32 },
    #[route("/stats")]
    StatsPage {},
    #[route("/users")]
    Users {},
}
#[component]
pub(crate) fn Nav(cx: Scope) -> Element {
    let mut user = use_shared_state::<User>(cx);
    if user.is_none() {
        use_shared_state_provider(cx, User::new);
        user = use_shared_state::<User>(cx);
    }
    if user?.read().Role.eq(&Role::Admin.to_string())
        || user?.read().Role.eq(&Role::Dev.to_string())
    {
        render_admin_nav(cx, user?)
    } else {
        render_user_nav(cx, user?)
    }
}

fn render_admin_nav<'a>(cx: Scope<'a>, user: &UseSharedState<User>) -> Element<'a> {
    let navigator = use_navigator(cx);
    if user.read().Token.is_empty() {
        navigator.push(Route::Login {});
    }

    render! {
        nav {
            div{
            }
            ul {
                li { Link { active_class:"active", to: Route::Home {}, "Bénéficiaires" } }
                li { Link { active_class:"active", to: Route::StatsPage {},"Statistiques" } }
                li { Link { active_class:"active", to: Route::Users {},"Utilisateurs" } }
            },

            div{
                button {
                    onclick: move |_| {
                    },
                    img{ src: "./icon/parameters.svg"},
                }
                Link { to: Route::Login {}, onclick: move |_| {
                    let beneficiaries = use_shared_state::<Beneficiaries>(cx).unwrap();
                    beneficiaries.write().beneficiaries.clear();
                },
                    img{ src: "./icon/logout.svg"},
                }
            }
        }
        Outlet::<Route> {}
    }
}

fn render_user_nav<'a>(cx: &'a Scoped<'a>, user: &UseSharedState<User>) -> Element<'a> {
    let navigator = use_navigator(cx);
    if user.read().Token.is_empty() {
        navigator.push(Route::Login {});
    }
    render! {
        nav {
            div{
            }
            Link { to: Route::Login {}, onclick: move |_| {
                let beneficiaries = use_shared_state::<Beneficiaries>(cx).unwrap();
                beneficiaries.write().beneficiaries.clear();
            },
                img{
                    class: "logout",
                    src: "./icon/logout.svg"
                },
            }

        }
        Outlet::<Route> {}
    }
}
