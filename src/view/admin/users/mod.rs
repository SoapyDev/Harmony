#![allow(non_snake_case)]

use crate::model::users::user::User;
use crate::model::users::user_role::UserRole;
use dioxus::prelude::*;

#[component]
pub fn Users(cx: Scope) -> Element {
    let user = use_shared_state::<User>(cx).unwrap();
    let users: &UseRef<Vec<UserRole>> = use_ref(cx, Vec::new);
    let users_loaded = use_state(cx, || false);

    let _ = use_future(cx, (), |_| {
        let user = user.clone();
        let users = users.clone();
        let users_loaded = users_loaded.clone();
        async move {
            let user_list = UserRole::get_all(user).await;
            users.with_mut(|u| *u = user_list);
            users_loaded.set(true);
        }
    });

    render! {
        style{
            include_str!("../../../assets/style/users.css"),
        }
        div{
            class: "create-button-container",
            CreateButton{
                users: users.clone(),
            },
        }
        if *users_loaded.get(){
            render!{
                div{
                    class: "users-container",
                    for u in users.read().iter() {
                        div{
                            key: "{u.Id}",
                            class: "user-row",
                            UsernameInput{
                                users: users.clone(),
                                user_role: u.clone(),
                            },
                            PasswordInput{
                                users: users.clone(),
                                user_role: u.clone(),
                            },
                            RoleInput{
                                users: users.clone(),
                                user_role: u.clone(),
                            },
                            DeleteButton{
                                users: users.clone(),
                                id: u.Id,
                            },
                        }
                    },
                }
            }
        },
    }
}

#[component]
fn UsernameInput(cx: Scope, users: UseRef<Vec<UserRole>>, user_role: UserRole) -> Element {
    let user = use_shared_state::<User>(cx).unwrap();

    render! {
        div{
            class: "input-container",
            label{
                "Nom d'utilisateur"
            }
            input{
                oninput: move |e| {
                   let value = e.value.clone();
                    users.with_mut(|u| {
                        let index = u.iter().position(|u| u.Id == user_role.Id);
                        if let Some(index) = index {
                            u[index].Username = value;
                        }
                    });
                },
                onchange: move |_| {
                    use_future(cx, (), |_| {
                        let user = user.clone();
                        let user_role = user_role.clone();
                        async move {
                            UserRole::update(user.clone(), user_role).await;
                        }
                    });
                },
                r#type: "text",
                value: "{user_role.Username}",
                class: "user_input",
            }
        }
    }
}

#[component]
fn PasswordInput(cx: Scope, users: UseRef<Vec<UserRole>>, user_role: UserRole) -> Element {
    let user = use_shared_state::<User>(cx).unwrap();

    render! {
        div{
            class: "input-container",
            label{
                "Mot de passe"
            }
            input{
                oninput: move |e| {
                   let value = e.value.clone();
                    users.with_mut(|u| {
                        let index = u.iter().position(|u| u.Id == user_role.Id);
                        if let Some(index) = index {
                            u[index].Password = value;
                        }
                    });
                },
                onchange: move |_| {
                    use_future(cx, (), |_| {
                        let user = user.clone();
                        let user_role = user_role.clone();
                        async move {
                            UserRole::update(user, user_role).await;
                        }
                    });
                },
                r#type: "password",
                min: 8,
                max: 50,
                value: "{user_role.Password}",
                class: "user_input",
                placeholder: "**********",
            }
        }
    }
}

#[component]
fn RoleInput(cx: Scope, users: UseRef<Vec<UserRole>>, user_role: UserRole) -> Element {
    let user = use_shared_state::<User>(cx).unwrap();

    render! {
        div{
            class: "input-container",
            label{
                "Rôle"
            },
            select{
                class: "select",
                oninput: move |e| {
                    let value = e.value.clone();
                    let index = users.read().iter().position(|u| u.Id == user_role.Id);

                    users.with_mut(|u| {
                        if let Some(index) = index {
                            u[index].Role = value;
                            use_future(cx, (), |_| {
                                let user = user.clone();
                                let u = u.clone();
                                async move {
                                    UserRole::update(user, u[index].clone()).await;
                                }
                            });
                        }
                    });
                },
                value: "{user_role.Role}",
                class: "user_input",
                option{
                    value: "Admin",
                    "Administrateur"
                },
                option{
                    value: "TS",
                    "Travailleur social"
                },
                option{
                    value: "User",
                    "Utilisateur"
                },
            },
        }
    }
}

#[component]
fn CreateButton(cx: Scope, users: UseRef<Vec<UserRole>>) -> Element {
    let user = use_shared_state::<User>(cx).unwrap();
    let use_clicked = use_state(cx, || false);
    let create_user = use_future(
        cx,
        (use_clicked, users, user),
        |(clic, users, user)| async move {
            if *clic {
                UserRole::create(user.clone(), users.clone()).await;
                let refresh = UserRole::get_all(user).await;
                users.with_mut(|u| *u = refresh);
                clic.set(false);
            }
        },
    );

    render! {
        button{
            class: "create-button",
            onclick: move |_| {
                use_clicked.set(true);
                create_user.restart();
            },
            "Nouvel utilisateur"
        }
    }
}

#[component]
fn DeleteButton(cx: Scope, id: i32, users: UseRef<Vec<UserRole>>) -> Element {
    let user = use_shared_state::<User>(cx).unwrap();
    let use_clicked = use_state(cx, || false);
    let _ = use_future(cx, use_clicked, |clic| {
        let user = user.clone();
        let users = users.clone();
        let id = *id;
        async move {
            if *clic {
                UserRole::delete(user, users, id).await;
                clic.set(false);
            }
        }
    });
    render! {
        label{
            ""
        },
        button{
            class: "delete-button",
            onclick: move |_| {
                use_clicked.set(true);
            },
            "Supprimer"
        }
    }
}
