#![allow(non_snake_case)]

use crate::controler::user::login;
use crate::model::users::user::User;
use dioxus_hooks::UseSharedState;
use dioxus_router::prelude::Navigator;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UserLogin {
    Username: String,
    Password: String,
}

impl UserLogin {
    pub fn new() -> UserLogin {
        UserLogin {
            Username: String::new(),
            Password: String::new(),
        }
    }
    pub fn set_username(&mut self, username: String) {
        self.Username = username;
    }
    pub fn set_password(&mut self, password: String) {
        self.Password = password;
    }
    pub fn is_valid(&self) -> bool {
        !self.Username.is_empty() && self.Password.len() >= 8
    }

    pub async fn login<'a>(
        &self,
        user: UseSharedState<User>,
        navigator: Navigator,
    ) -> Result<(), anyhow::Error> {
        let res = login(self).await;
        match res {
            Ok(other) => {
                user.with_mut(|val| {
                    val.Token = other.Token;
                    val.Role = other.Role;
                });
                navigator.push("/home");
            },
            Err(e) => {
                println!("{:?}", e);
            }
        }
        Ok(())
    }
}
