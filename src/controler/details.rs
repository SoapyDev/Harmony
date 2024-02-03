#![allow(non_snake_case)]

use crate::controler::connection::ConnectionUrls;
use crate::model::beneficiaries::details::{Allergy, Detail, Presence};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub(crate) struct TokenPresence {
    pub(crate) Token: String,
    pub(crate) Presence: Presence,
}



#[derive(Clone, Serialize)]
pub(crate) struct TokenAllergy {
    pub(crate) Token: String,
    pub(crate) Allergy: Allergy,
}


impl Detail {
    pub(crate) async fn insert_presence(
        token_presence: TokenPresence,
    ) -> Result<(), anyhow::Error> {
        let res = reqwest::Client::new()
            .post(ConnectionUrls::InsertPresence.to_string())
            .json(&token_presence)
            .send()
            .await?;
        match res.status() {
            reqwest::StatusCode::OK => Ok(()),
            _ => Err(anyhow::Error::msg("Failed to insert presence")),
        }
    }
    pub(crate) async fn delete_presence(
        token_presence: TokenPresence,
    ) -> Result<(), anyhow::Error> {
        let res = reqwest::Client::new()
            .post(ConnectionUrls::DeletePresence.to_string())
            .json(&token_presence)
            .send()
            .await?;
        match res.status() {
            reqwest::StatusCode::OK => Ok(()),
            _ => {
                Err(anyhow::Error::msg("Failed to delete presence"))
            }
        }
    }
    
    pub(crate) async fn insert_allergy(
        token_allergy: TokenAllergy
    ) -> Result<(), anyhow::Error>{
        let res = reqwest::Client::new()
            .post(ConnectionUrls::InsertAllergy.to_string())
            .json(&token_allergy)
            .send()
            .await?;
        match res.status() {
            reqwest::StatusCode::OK => Ok(()),
            _ => {
                Err(anyhow::Error::msg("Failed to insert allergy"))
            }
        }
    }
    
    pub(crate) async fn delete_allergy(
        token_allergy: TokenAllergy
    ) -> Result<(), anyhow::Error>{
        let res = reqwest::Client::new()
            .post(ConnectionUrls::DeleteAllergy.to_string())
            .json(&token_allergy)
            .send()
            .await?;
        match res.status() {
            reqwest::StatusCode::OK => Ok(()),
            _ => {
                Err(anyhow::Error::msg("Failed to delete allergy"))
            }
        }
    }
}
