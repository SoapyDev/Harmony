use crate::controler::connection::{
    ConnectionUrls, Token, TokenBeneficiary, TokenBeneficiaryId, TokenSearch,
};
use crate::model::beneficiaries::beneficiary::{Beneficiaries, Beneficiary};
use crate::model::beneficiaries::details::Detail;
use bincode::{config, decode_from_slice};
use dioxus_desktop::wry::http::StatusCode;

impl Beneficiaries {
    pub(crate) async fn get_all_beneficiary(
        token: Token,
    ) -> Result<Vec<Beneficiary>, anyhow::Error> {
        let bene = reqwest::Client::new()
            .post(ConnectionUrls::SelectBeneficiaries.to_string())
            .json(&token)
            .send()
            .await?;

        if bene.status() != StatusCode::OK {
            return Err(anyhow::Error::msg("Quelque chose s'est mal passé"));
        }

        let config = config::standard();
        let bytes = bene.bytes().await?;

        let decoded = tokio::task::spawn_blocking(move || {
            let (decoded, _len): (Vec<Beneficiary>, usize) =
                decode_from_slice(bytes.as_ref(), config)
                    .unwrap_or((vec![], 0));
            decoded
        })
        .await?;
        Ok(decoded)
    }
    pub(crate) async fn search(
        token_search: TokenSearch,
    ) -> Result<Vec<Beneficiary>, anyhow::Error> {
        let bene = reqwest::Client::new()
            .post(ConnectionUrls::SearchBeneficiary.to_string())
            .json(&token_search)
            .send()
            .await?;

        if bene.status() != StatusCode::OK {
            return Err(anyhow::Error::msg("Quelque chose s'est mal passé"));
        }

        let config = config::standard();
        let (decoded, _len): (Vec<Beneficiary>, usize) =
            decode_from_slice(&bene.bytes().await?, config)?;
        Ok(decoded)
    }
}
impl Beneficiary {
    pub(crate) async fn create(token: Token) -> Result<Self, anyhow::Error> {
        let bene = reqwest::Client::new()
            .post(ConnectionUrls::CreateBeneficiary.to_string())
            .json(&token)
            .send()
            .await?;

        if bene.status() != StatusCode::OK {
            return Err(anyhow::Error::msg("Quelque chose s'est mal passé"));
        }

        let config = config::standard();
        let (decoded, _len): (Beneficiary, usize) =
            decode_from_slice(&bene.bytes().await?, config)?;
        Ok(decoded)
    }

    pub(crate) async fn update_beneficiary(
        token_bene: TokenBeneficiary,
    ) -> Result<(), anyhow::Error> {
        let bene = reqwest::Client::new()
            .post(ConnectionUrls::UpdateBeneficiary.to_string())
            .json(&token_bene)
            .send()
            .await?;

        if bene.status() != StatusCode::OK {
            return Err(anyhow::Error::msg("Quelque chose s'est mal passé"));
        }

        match bene.status() {
            StatusCode::OK => Ok(()),
            StatusCode::INTERNAL_SERVER_ERROR => Err(anyhow::Error::msg("Internal server error")),
            _ => Err(anyhow::Error::msg("Unknown error")),
        }
    }

    pub(crate) async fn get_beneficiary_details(
        token_bene: TokenBeneficiaryId,
    ) -> Result<(Self, Detail), anyhow::Error> {
        let bene = reqwest::Client::new()
            .post(ConnectionUrls::SelectBeneficiary.to_string())
            .json(&token_bene)
            .send()
            .await?;

        if bene.status() != StatusCode::OK {
            return Err(anyhow::Error::msg("Quelque chose s'est mal passé"));
        }

        let config = config::standard();
        let (decoded, _len): ((Beneficiary, Detail), usize) =
            decode_from_slice(&bene.bytes().await?, config)?;
        Ok(decoded)
    }
}
