#![allow(non_snake_case)]

use crate::controler::details::TokenPresence;
use crate::model::users::user::User;
use bincode::{Decode, Encode};
use dioxus_hooks::{UseRef, UseSharedState};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Encode, Decode, Serialize, Deserialize, Default, Clone)]
pub(crate) struct Detail {
    pub(crate) id: i32,
    pub(crate) presences: Vec<Presence>,
    pub(crate) allergies: Vec<Allergy>,
}

impl Detail {
    pub(crate) fn get_allergies(&self) -> Vec<String> {
        self.allergies
            .iter()
            .map(|a| a.Allergy.clone())
            .collect::<Vec<String>>()
    }

    pub(crate) async fn push(
        user: UseSharedState<User>,
        detail: &UseRef<Detail>,
        presence: Presence,
    ) -> bool {
        if detail
            .read()
            .presences
            .iter()
            .any(|d| d.Date == presence.Date)
        {
            return false;
        }
        let token_presence = TokenPresence {
            Token: user.read().session.clone(),
            Presence: presence,
        };
        if Self::insert_presence(token_presence).await.is_ok() {
            return true;
        }
        false
    }

    pub(crate) async fn pop(
        user: UseSharedState<User>,
        detail: &UseRef<Detail>,
        presence: Presence,
    ) -> bool {
        if !detail
            .read()
            .presences
            .iter()
            .any(|d| d.Date == presence.Date)
        {
            return false;
        }

        let token_presence = TokenPresence {
            Token: user.read().session.clone(),
            Presence: presence,
        };

        if Self::delete_presence(token_presence).await.is_ok() {
            return true;
        }
        false
    }
}
#[derive(Encode, Decode, Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Presence {
    pub(crate) BeneficiaryId: i32,
    pub(crate) Date: String,
}
#[derive(Encode, Decode, Serialize, Deserialize, Clone)]
pub(crate) struct Allergy {
    pub(crate) BeneficiaryId: i32,
    pub(crate) Allergy: String,
}

impl Display for Allergy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.Allergy)
    }
}
