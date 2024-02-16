use crate::controler::connection::{ConnectionUrls, Token};
use crate::model::stats::stats::{Stats, StatsRes};
use bincode::{config, decode_from_slice};
use ConnectionUrls::SelectStats;

impl Stats {
    pub(crate) async fn get_stats(token: Token) -> Result<Stats, anyhow::Error> {
        let res = reqwest::Client::new()
            .post(SelectStats.to_string())
            .json(&token)
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(anyhow::anyhow!("Failed to get stats"));
        }

        let config = config::standard();
        let (decoded, _len): (StatsRes, usize) = decode_from_slice(&res.bytes().await?, config)?;
        let stats = Stats::from(decoded);
        Ok(stats)
    }
}
