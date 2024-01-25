use crate::controler::connection::{ConnectionUrls, Token, TokenBeneficiary, TokenBeneficiaryId, TokenSearch};
use crate::model::beneficiaries::beneficiary::{Beneficiaries, Beneficiary};
use crate::model::beneficiaries::details::Detail;
use bincode::{config, decode_from_slice};
use dioxus_desktop::wry::http::StatusCode;
use log::{error, info};

impl Beneficiaries {
    pub(crate) async fn get_all_beneficiary(
        token: Token,
    ) -> Result<Vec<Beneficiary>, anyhow::Error> {
        info!("Get all beneficiaries");
        let bene = reqwest::Client::new()
            .post(ConnectionUrls::GetBeneficiaries.to_string())
            .json(&token)
            .send()
            .await?;

        if bene.status() != StatusCode::OK {
            error!("Error : {}", bene.status());
            return Err(anyhow::Error::msg("Quelque chose s'est mal passé"));
        }
        info!("Get all beneficiaries : {}", bene.status());

        let config = config::standard();
        let bytes = bene.bytes().await?;

        let decoded = tokio::task::spawn_blocking(move || {
            let (decoded, _len): (Vec<Beneficiary>, usize) =
                decode_from_slice(bytes.as_ref(), config)
                    .map_err(|e| {
                        error!("Failed to decode beneficiaries : {}", e);
                        e
                    })
                    .unwrap_or((vec![], 0));
            decoded
        })
        .await?;
        Ok(decoded)
    }
    pub(crate) async fn search(token_search: TokenSearch) -> Result<Vec<Beneficiary>, anyhow::Error> {
        info!("Search beneficiary");
        let bene = reqwest::Client::new()
            .post(ConnectionUrls::SearchBeneficiary.to_string())
            .json(&token_search)
            .send()
            .await?;

        if bene.status() != StatusCode::OK {
            error!("Error : {}", bene.status());
            return Err(anyhow::Error::msg("Quelque chose s'est mal passé"));
        }

        info!("Search beneficiary : {}", bene.status());
        let config = config::standard();
        let (decoded, _len): (Vec<Beneficiary>, usize) =
            decode_from_slice(&bene.bytes().await?, config)?;
        Ok(decoded)
    }
}
impl Beneficiary {
    pub(crate) async fn create_new(token: Token) -> Result<Self, anyhow::Error> {
        info!("Create new beneficiary");
        let bene = reqwest::Client::new()
            .post(ConnectionUrls::CreateBeneficiary.to_string())
            .json(&token)
            .send()
            .await?;

        if bene.status() != StatusCode::OK {
            error!("Error : {}", bene.status());
            return Err(anyhow::Error::msg("Quelque chose s'est mal passé"));
        }

        info!("Create new beneficiary : {}", bene.status());

        let config = config::standard();
        let (decoded, _len): (Beneficiary, usize) =
            decode_from_slice(&bene.bytes().await?, config)?;
        Ok(decoded)
    }

    pub(crate) async fn update_beneficiary(
        token_bene: TokenBeneficiary,
    ) -> Result<(), anyhow::Error> {
        info!("Update beneficiary");
        let bene = reqwest::Client::new()
            .post(ConnectionUrls::UpdateBeneficiary.to_string())
            .json(&token_bene)
            .send()
            .await?;

        if bene.status() != StatusCode::OK {
            error!("Error : {}", bene.status());
            return Err(anyhow::Error::msg("Quelque chose s'est mal passé"));
        }

        info!("Update beneficiary : {}", bene.status());

        match bene.status() {
            StatusCode::OK => Ok(()),
            StatusCode::INTERNAL_SERVER_ERROR => Err(anyhow::Error::msg("Internal server error")),
            _ => Err(anyhow::Error::msg("Unknown error")),
        }
    }

    pub(crate) async fn get_beneficiary_details(
        token_bene: TokenBeneficiaryId,
    ) -> Result<(Self, Detail), anyhow::Error> {
        info!("Get beneficiary details : {}", token_bene.Id);
        let bene = reqwest::Client::new()
            .post(ConnectionUrls::GetBeneficiary.to_string())
            .json(&token_bene)
            .send()
            .await?;

        if bene.status() != StatusCode::OK {
            error!("Error : {}", bene.status());
            return Err(anyhow::Error::msg("Quelque chose s'est mal passé"));
        }

        info!("Get beneficiary details : {}", bene.status());
        let config = config::standard();
        let (decoded, _len): ((Beneficiary, Detail), usize) =
            decode_from_slice(&bene.bytes().await?, config)?;
        Ok(decoded)
    }
    

}
