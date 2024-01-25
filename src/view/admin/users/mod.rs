use crate::model::users::user::User;
use crate::model::users::user_role::UserRole;
use dioxus::prelude::*;

#[component]
pub(crate) fn Users(cx: Scope) -> Element {
    let user = use_shared_state::<User>(cx).unwrap();
    let users: &UseRef<Vec<UserRole>> = use_ref(cx, Vec::new);

    render! {
        section{
            div{
                h2{"Utilisateurs"},
            },
            table{
                thead{
                    tr{
                        th{"Nom d'utilisateurs"},
                        th{"Mot de passe"},
                        th{"Rôle"},
                        th{"Nouveau"},
                        th{"Editer"},
                        th{"Supprimer"},
                    }
                },
                tbody{
                    for user in users.read().iter(){
                        tr{
                            td{user.username.clone()},
                            td{user.password.clone()},
                            td{user.role.clone()},
                            td{
                                button{
                                    value :"+",
                                    disabled: true,
                                }
                            },
                            td{
                                button{
                                    value : "o",
                                    disabled: true,
                                }
                            },
                            td{
                                button{
                                    value: "-",
                                    disabled: true,
                                }
                            },
                        }
                    }
                }

            }
        }
    }
}
