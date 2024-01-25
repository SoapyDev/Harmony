use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(PartialEq, Clone)]
pub enum Role {
    User,
    Admin,
    TS,
    Dev,
    None,
}

impl Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin"),
            Role::TS => write!(f, "TS"),
            Role::Dev => write!(f, "Dev"),
            Role::None => write!(f, "None"),
        }
    }
}

impl FromStr for Role {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "User" => Ok(Role::User),
            "Admin" => Ok(Role::Admin),
            "TS" => Ok(Role::TS),
            "Dev" => Ok(Role::Dev),
            _ => Ok(Role::None),
        }
    }
}
