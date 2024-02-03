#![allow(non_snake_case)]

use crate::model::users::user::User;
use crate::model::users::user_login::UserLogin;
use crate::view::inputs::logins::{Password, SubmitButton, Username};
use dioxus::hooks::{use_future, use_ref, use_shared_state};
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;

pub(crate) fn Login(cx: Scope) -> Element {
    let user = use_ref(cx, UserLogin::new);
    let user_token = use_shared_state::<User>(cx).unwrap();
    let navigator = use_navigator(cx);

    render!(
        style{
            include_str!("../../assets/style/login.css")
        }
        div {
            class: "login-container",
            div {
                class: "messages",
                h2{
                    span{"\" Hey! I just met you"},
                    br{},
                    span{"And this is crazy"},
                },
                h2{
                    span{"But here's a form"},
                    br{},
                    span{"So fill it, maybe ? \""},
                },
            },
            div {
                class: "form-container",
                header{
                    img{ src: "./src/assets/icon/asbc.svg" },
                }
                form {
                    onsubmit: move |evt| {
                        evt.stop_propagation();
                        if user.read().is_valid(){
                            let _ = use_future(cx, (), |_| {
                                let user = user.read().clone();
                                let user_token = user_token.clone();
                                let navigator = navigator.clone();
                                async move {
                                    user.login(user_token, navigator).await
                                }
                            });
                        }
                    },
                    Username{
                        min: 2,
                        max: 45,
                        label: "Nom d'utilisateur",
                        user: user.clone(),
                    }
                    Password{
                        min: 8,
                        max: 45,
                        label: "Mot de passe",
                        user: user.clone(),
                    }
                    SubmitButton{
                       label : "Connexion",
                    }

                }
            }
        },
    )
}
