use std::str::FromStr;

use anyhow::Result;
// use serde_json::{json, to_string_pretty};
use shared_crypto::intent::{Intent, IntentMessage};
use sui_sdk::{
    rpc_types::SuiTransactionBlockResponseOptions,
    types::{
        base_types::SuiAddress,
        crypto::{Signature, SuiKeyPair},
        messages::Transaction,
    },
    SuiClientBuilder,
};

use tokio::time;

pub struct Bot {
    interval: u64,
    keypair: SuiKeyPair,
}

impl Bot {
    pub fn new(interval: u64, pk: &str) -> Result<Self> {
        let bot = Self {
            interval,
            keypair: SuiKeyPair::from_str(pk).unwrap(),
        };

        Ok(bot)
    }

    pub async fn start(&self) {
        let duration = time::Duration::from_secs(self.interval);
        let mut interval = time::interval(duration);

        let address: SuiAddress = (&self.keypair.public()).into();
        let client = SuiClientBuilder::default()
            .build("https://fullnode.testnet.sui.io:443")
            .await
            .unwrap();
        let client_coin_read = client.coin_read_api();

        let coins = client_coin_read
            .get_all_coins(address, None, None)
            .await
            .unwrap();

        let coin = coins.data.get(0).unwrap();

        let txb = client.transaction_builder();
        let data = txb
            .transfer_object(
                address,
                coin.coin_object_id,
                None,
                20000000,
                SuiAddress::from_str(
                    "0x02be9f4658200072e5c4e48a8eb25a770644d8ecba6f439740893d20b67b674e",
                )
                .unwrap(),
            )
            .await
            .unwrap();

        let signature = Signature::new_secure(
            &IntentMessage::new(Intent::sui_transaction(), &data),
            &self.keypair,
        );

        let r = client
            .quorum_driver()
            .execute_transaction_block(
                Transaction::from_data(data, Intent::sui_transaction(), vec![signature])
                    .verify()
                    .unwrap(),
                SuiTransactionBlockResponseOptions::default(),
                None,
            )
            .await
            .unwrap();

        println!("{:?}", r);

        // println!("{}", to_string_pretty(&json!(coins)).unwrap());

        loop {
            interval.tick().await;
        }
    }

    // async fn get_balance(&self, address: &str) -> Vec<SuiObjectResponse> {
    //     let builder = SuiClientBuilder::default();
    //     let client = builder
    //         .build("https://fullnode.devnet.sui.io:443")
    //         .await
    //         .unwrap();

    //     let intent = IntentMessage::new(Intent::sui_transaction(), "value");
    //     Signature::new_secure(&intent, &self.keypair);

    //     let resp = client
    //         .read_api()
    //         .get_owned_objects(SuiAddress::from_str(address).unwrap(), None, None, None)
    //         .await
    //         .unwrap();

    //     resp.data
    // }
}
