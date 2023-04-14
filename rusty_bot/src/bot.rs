use std::str::FromStr;

use anyhow::Result;
use sui_sdk::types::crypto::SuiKeyPair;

use tokio::time;

use crate::strategy::Strategy;

pub struct Bot {
    interval: u64,
    _keypair: SuiKeyPair,
    strategy: Strategy,
}

impl Bot {
    pub fn new(interval: u64, pk: &str) -> Result<Self> {
        let bot = Self {
            interval,
            _keypair: SuiKeyPair::from_str(pk).expect("Unable to parse pk"),
            strategy: Strategy::package(
                "0x7a21bdbf402b2807ba8d92fac6fdf8a1278c6568ac4965752b45246580a79a17",
                "0xdb9e28d8ca84d419fedd2b1891481f19901deffdba50377c181194895a26609b",
            )?,
        };

        Ok(bot)
    }

    pub async fn start(&self) -> Result<()> {
        let duration = time::Duration::from_secs(self.interval);
        let mut interval = time::interval(duration);

        loop {
            interval.tick().await;

            match &self.strategy {
                Strategy::Package(package) => package.buy_cheapest_item().await?,
            };
        }
    }
}
