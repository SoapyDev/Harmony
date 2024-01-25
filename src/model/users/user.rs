use crate::model::users::role::Role;
use bincode::Decode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Decode, Clone)]
pub struct User {
    pub(crate) session: String,
    pub(crate) role: String,
}

impl User {
    pub fn new() -> User {
        User {
            session: String::new(),
            role: Role::None.to_string(),
        }
    }
    pub fn is_connected(&self) -> bool {
        !self.session.is_empty() && self.role != Role::None.to_string()
    }
}
