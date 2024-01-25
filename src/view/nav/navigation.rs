use crate::model::beneficiaries::beneficiary::Beneficiaries;
use crate::model::users::{role::Role, user::User};
use crate::view::admin::{stats::StatsPage, users::Users};
use crate::view::login::Login;
use crate::view::nav::export::export;
use crate::view::nav::home::Home;
use crate::view::ts::beneficiary::BeneficiaryPage;
use dioxus::prelude::*;
use dioxus_router::components::{Link, Outlet};
use dioxus_router::hooks::use_navigator;
use dioxus_router::prelude::Routable;
use log::error;

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
        error!("Could not find a user in the shared state, creating a new user");
        use_shared_state_provider(cx, User::new);
        user = use_shared_state::<User>(cx);
    }
    if user?.read().role.eq(&Role::Admin.to_string())
        || user?.read().role.eq(&Role::Dev.to_string())
    {
        render_admin_nav(cx, user?)
    } else {
        render_user_nav(cx, user?)
    }
}

fn render_admin_nav<'a>(cx: Scope<'a>, user: &UseSharedState<User>) -> Element<'a> {
    let navigator = use_navigator(cx);
    if user.read().session.is_empty() {
        navigator.push(Route::Login {});
    }
    render! {
        nav {
            div{
            }
            ul {
                li { Link { active_class:"active", to: Route::Home {}, "Beneficiaires" } }
                li { Link { active_class:"active", to: Route::StatsPage {},"Statistiques" } }
                li { Link { active_class:"active", to: Route::Users {},"Utilisateurs" } }
            },

            div{
                button {
                    img{ src: "./src/assets/icon/notifications.svg"},
                }
                button {
                    onclick: move |_| {
                        if export(cx).is_err(){
                           error!("Could not export the data due to : {:?}", export(cx).err());
                        };
                    },
                    img{ src: "./src/assets/icon/parameters.svg"},
                }
                Link { to: Route::Login {}, onclick: move |_| {
                    let beneficiaries = use_shared_state::<Beneficiaries>(cx).unwrap();
                    beneficiaries.write().beneficiaries.clear();
                },
                    img{ src: "./src/assets/icon/logout.svg"},
                }
            }
        }
        Outlet::<Route> {}
    }
}

fn render_user_nav<'a>(cx: &'a Scoped<'a>, user: &UseSharedState<User>) -> Element<'a> {
    let navigator = use_navigator(cx);
    if user.read().session.is_empty() {
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
                    src: "./src/assets/icon/logout.svg"
                },
            }

        }
        Outlet::<Route> {}
    }
}
