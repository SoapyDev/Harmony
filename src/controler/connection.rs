#![allow(non_snake_case)]

use crate::model::beneficiaries::beneficiary::Beneficiary;
use crate::model::users::user::User;
use bincode::Decode;
use dioxus_hooks::UseSharedState;
use dotenv::var;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub(crate) enum ConnectionUrls {
    Login,
    GetUsers,
    CreateUser,
    UpdateUser,
    DeleteUser,
    GetBeneficiaries,
    GetBeneficiary,
    SearchBeneficiary,
    CreateBeneficiary,
    UpdateBeneficiary,
    InsertPresence,
    DeletePresence,
    InsertAllergy,
    DeleteAllergy,
    GetStats,
}

impl Display for ConnectionUrls {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let base_url = var("CONNECTION_URL").unwrap();
        match self {
            ConnectionUrls::Login => write!(f, "{}/login", base_url),
            ConnectionUrls::GetUsers => write!(f, "{}/getUsers", base_url),
            ConnectionUrls::CreateUser => write!(f, "{}/createUser", base_url),
            ConnectionUrls::UpdateUser => write!(f, "{}/updateUser", base_url),
            ConnectionUrls::DeleteUser => write!(f, "{}/deleteUser", base_url),
            ConnectionUrls::GetBeneficiaries => write!(f, "{}/getBeneficiaries", base_url),
            ConnectionUrls::GetBeneficiary => write!(f, "{}/getBeneficiary", base_url),
            ConnectionUrls::SearchBeneficiary => write!(f, "{}/searchBeneficiaries", base_url),
            ConnectionUrls::CreateBeneficiary => write!(f, "{}/createBeneficiary", base_url),
            ConnectionUrls::UpdateBeneficiary => write!(f, "{}/updateBeneficiary", base_url),
            ConnectionUrls::InsertPresence => write!(f, "{}/insertPresence", base_url),
            ConnectionUrls::DeletePresence => write!(f, "{}/deletePresence", base_url),
            ConnectionUrls::InsertAllergy => write!(f, "{}/insertAllergy", base_url),
            ConnectionUrls::DeleteAllergy => write!(f, "{}/deleteAllergy", base_url),
            ConnectionUrls::GetStats => write!(f, "{}/getStats", base_url),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Decode)]
pub(crate) struct Token {
    pub(crate) Token: String,
}

impl Token {
    pub(crate) fn new(token: String) -> Self {
        Self { Token: token }
    }
}

impl From<UseSharedState<User>> for Token {
    fn from(value: UseSharedState<User>) -> Self {
        Self::new(value.read().Token.clone())
    }
}

#[derive(Clone, Serialize, Deserialize, Decode)]
pub(crate) struct TokenBeneficiary {
    pub(crate) Token: String,
    pub(crate) Beneficiary: Beneficiary,
}

impl TokenBeneficiary {
    pub(crate) fn new(token: Token, beneficiary: Beneficiary) -> Self {
        Self {
            Token: token.Token,
            Beneficiary: beneficiary,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Decode)]
pub(crate) struct TokenBeneficiaryId {
    pub(crate) Token: String,
    pub(crate) Id: i32,
}

impl TokenBeneficiaryId {
    pub(crate) fn new(token: Token, id: i32) -> Self {
        Self {
            Token: token.Token,
            Id: id,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Decode)]
pub(crate) struct TokenSearch {
    pub(crate) Token: String,
    pub(crate) Search: String,
}
