#![allow(non_snake_case)]

use crate::model::users::role::Role;
use bincode::Decode;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Decode, Clone)]
pub struct User {
    pub(crate) Token: String,
    pub(crate) Role: String,
}

impl User {
    pub fn new() -> User {
        User {
            Token: String::new(),
            Role: Role::None.to_string(),
        }
    }
    pub fn is_connected(&self) -> bool {
        !self.Token.is_empty() && self.Role != Role::None.to_string()
    }
}
