use crate::controler::connection::Token;
use crate::controler::user::{create_user, delete_user, get_users, update_user, UserToken};
use crate::model::users::user::User;
use bincode::Decode;
use dioxus::core::Scope;
use dioxus_hooks::{UseRef, UseSharedState};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Decode, Serialize, Clone, PartialEq)]
pub(crate) struct UserRole {
    pub(crate) Id: i32,
    pub(crate) Username: String,
    pub(crate) Password: String,
    pub(crate) Role: String,
}

impl UserRole {
    pub(crate) fn new() -> UserRole {
        UserRole {
            Id: 0,
            Username: String::new(),
            Password: String::new(),
            Role: String::new(),
        }
    }

    pub(crate) async fn get_all(user: UseSharedState<User>) -> Vec<UserRole> {
        let token = Token::from(user);
        let users = get_users(token).await;

        if users.is_err() {
            return Vec::new();
        }

        users.unwrap()
    }

    pub(crate) async fn create(user: UseSharedState<User>, users: UseRef<Vec<UserRole>>) {
        let new_user = UserRole {
            Id: 0,
            Username: format!(
                "utilisateur-{}{}",
                users.read().len() + 3,
                thread_rng().gen_range(0..1000000)
            ),
            Password: String::from("pa$$w0rd"),
            Role: "User".to_string(),
        };
        let user_token = UserToken {
            Token: user.read().Token.clone(),
            User: new_user,
        };

        let res = create_user(user_token).await;

        if res.is_err() {
            return;
        }

        users.with_mut(|users| users.push(res.unwrap()));
    }

    pub(crate) async fn update(user: UseSharedState<User>, other: UserRole) {
        let user_token = UserToken {
            Token: user.read().Token.clone(),
            User: other.clone(),
        };

        let _ = update_user(user_token).await;
    }

    pub(crate) async fn delete(
        user: UseSharedState<User>,
        users: UseRef<Vec<UserRole>>,
        other: i32,
    ) {
        let user_delete = UserRole {
            Id: other,
            Username: String::new(),
            Password: String::new(),
            Role: String::new(),
        };

        let user_token = UserToken {
            Token: user.read().Token.clone(),
            User: user_delete,
        };

        let res = delete_user(user_token).await;

        if res.is_err() {
            return;
        }

        users.with_mut(|users| users.retain(|u| u.Id != other));
    }
}
