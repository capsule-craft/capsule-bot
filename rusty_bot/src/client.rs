use anyhow::Result;
use sui_sdk::{SuiClient, SuiClientBuilder};

pub async fn sui_client() -> Result<SuiClient> {
    let client = SuiClientBuilder::default()
        .build("https://fullnode.testnet.sui.io:443")
        .await?;

    Ok(client)
}
