use crate::controler::connection::{ConnectionUrls, Token};
use crate::model::stats::stats::{Stats, StatsRes};
use bincode::{config, decode_from_slice};
use log::{error, info};
use ConnectionUrls::GetStats;

impl Stats {
    pub(crate) async fn get_stats(token: Token) -> Result<Stats, anyhow::Error> {
        info!("Getting stats");
        let res = reqwest::Client::new()
            .post(GetStats.to_string())
            .json(&token)
            .send()
            .await?;

        if !res.status().is_success() {
            error!("Failed to get stats");
            return Err(anyhow::anyhow!("Failed to get stats"));
        }

        info!("Get stats : {}", res.status());
        let config = config::standard();
        let (decoded, _len): (StatsRes, usize) = decode_from_slice(&res.bytes().await?, config)?;
        let stats = Stats::from(decoded);
        Ok(stats)
    }
}
