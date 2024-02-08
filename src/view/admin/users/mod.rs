#![allow(non_snake_case)]

use crate::model::users::user::User;
use crate::model::users::user_role::UserRole;
use dioxus::prelude::*;

#[component]
pub fn Users(cx: Scope) -> Element {
    let user = use_shared_state::<User>(cx).unwrap();
    let users: &UseRef<Vec<UserRole>> = use_ref(cx, Vec::new);
    let is_to_update = use_state(cx, || false);
    let current_id = use_state(cx, || 0);
    let current_user = use_ref(cx, UserRole::new);

    let get_user = use_future(cx, user, |user| {
        let users = users.clone();
        async move {
            let res = UserRole::get_all(user).await;
            users.with_mut(|u| *u = res);
        }
    });

    let _ = use_future(cx, current_id, |current| {
        let users = users.clone();
        let current_user = current_user.clone();
        async move {
            if *current.get() == 0 {
                current_user.set(UserRole::new());
                return;
            }
            let user = users
                .read()
                .iter()
                .find(|u| u.Id == *current.get())
                .unwrap()
                .clone();
            current_user.set(user);
        }
    });

    let update_user = use_future(cx, (user, is_to_update), |(user, is_to_update)| {
        let users = users.clone();
        let current = current_user.read().clone();
        let current_id = current_id.clone();
        async move {
            if !*is_to_update {
                return;
            }
            UserRole::update(user.clone(), current).await;
            let refresh = UserRole::get_all(user).await;
            users.with_mut(|u| *u = refresh);
            is_to_update.set(false);
            current_id.set(0);
        }
    });

    let reader = users.read().clone();

    render! {
        style{
            include_str!("../../../assets/style/users.css"),
        },
        div{
            class: "users",
            h1{
                "Utilisateurs"
            },
            table{
                thead {
                    tr{
                        th{
                            "Nom d'utilisateur",
                        },
                        th{
                            "Mot de passe",
                        },
                        th{
                            "Rôle",
                        },
                        th{
                            CreateButton {
                                users: users.clone(),
                                id: current_id.clone(),
                            }
                        }
                    }
                },
                tbody {
                    for element in reader{
                        tr{
                            key: "{element.Id}",
                            ondblclick : move |_| {
                                current_id.set(element.Id);
                            },
                            if *current_id.get() == element.Id {
                                render!{
                                    td{
                                        input{
                                            oninput: move |event| {
                                                current_user.with_mut(|u| u.Username = event.value.clone());
                                            },
                                            r#type: "text",
                                            value: "{current_user.read().Username}",
                                            placeholder: "Nom d'utilisateur",
                                        },
                                    },
                                    td{
                                        input{
                                            oninput: move |event| {
                                                current_user.with_mut(|u| u.Password = event.value.clone());
                                            },
                                            r#type: "password",
                                            placeholder: "*******************",
                                            value: "{current_user.read().Password}",
                                        },
                                    },
                                    td{
                                        select{
                                            class: "select",
                                            onchange: move |event| {
                                                current_user.with_mut(|u| u.Role = event.value.clone());
                                            },
                                            value: "{current_user.read().Role}",
                                            option{
                                                value: "User",
                                                "Utilisateur"
                                            },
                                            option{
                                                value: "Admin",
                                                "Administrateur"
                                            },
                                            option {
                                                value: "TS",
                                                "Travailleur Social"
                                            },
                                        },
                                    },
                                    td{
                                        button{
                                            class: "button",
                                            onclick: move |_| {
                                                is_to_update.set(true);
                                            },
                                            img{
                                                class: "update",
                                                src: "./icon/update.svg"
                                            }
                                        },
                                        DeleteButton {
                                            id: element.Id,
                                            users: users.clone(),
                                            current_id: current_id.clone(),
                                        }
                                    }
                                }
                            }else{
                                render!{
                                    td{
                                        "{element.Username}"
                                    },
                                    td{
                                        "*******************"
                                    },
                                    td{
                                        "{user_role(element.Role)}"
                                    },
                                    td{
                                        DeleteButton {
                                            id: element.Id,
                                            users: users.clone(),
                                            current_id: current_id.clone(),
                                        }
                                    }
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CreateButton(cx: Scope, users: UseRef<Vec<UserRole>>, id: UseState<i32>) -> Element {
    let user = use_shared_state::<User>(cx).unwrap();
    let use_clicked = use_state(cx, || false);
    let create_user = use_future(cx, (use_clicked, users, user), |(click, users, user)| {
        let id = id.clone();
        async move {
            if *click {
                UserRole::create(user.clone(), users.clone()).await;
                let refresh = UserRole::get_all(user).await;
                users.with_mut(|u| *u = refresh);
                let last = users.read().last().unwrap().Id;
                id.set(last);
                click.set(false);
            }
        }
    });

    render! {
        button{
            class: "button",
            onclick: move |_| {
                use_clicked.set(true);
                create_user.restart();
            },
            img{
                class: "create",
                src: "./icon/create.svg"
            }
        }
    }
}

#[component]
fn DeleteButton(
    cx: Scope,
    id: i32,
    users: UseRef<Vec<UserRole>>,
    current_id: UseState<i32>,
) -> Element {
    let user = use_shared_state::<User>(cx).unwrap();
    let use_clicked = use_state(cx, || false);
    let _ = use_future(cx, use_clicked, |clic| {
        let current_id = current_id.clone();
        let user = user.clone();
        let users = users.clone();
        let id = *id;
        async move {
            if *clic {
                UserRole::delete(user, users, id).await;
                if *current_id.get() == id {
                    current_id.set(0);
                }
                clic.set(false);
            }
        }
    });
    render! {
        label{
            ""
        },
        button{
            class: "button",
            onclick: move |_| {
                use_clicked.set(true);
            },
            img{
                class: "delete",
                src: "./icon/delete.svg"
            }
        }
    }
}

fn user_role(user: String) -> String {
    match user.as_str() {
        "User" => "Utilisateur".to_string(),
        "Admin" => "Administrateur".to_string(),
        "TS" => "Travailleur Social".to_string(),
        _ => "Utilisateur".to_string(),
    }
}
