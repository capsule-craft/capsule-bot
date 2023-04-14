use std::str::FromStr;

use anyhow::{bail, Ok, Result};
use serde::{Deserialize, Serialize};
use sui_sdk::{
    rpc_types::{SuiObjectDataOptions, SuiRawData},
    types::{
        base_types::{ObjectID, SuiAddress},
        id::UID,
    },
};

use crate::client::sui_client;

pub struct Package {
    _package_id: ObjectID,
    registry_id: ObjectID,
}

#[derive(Debug, Serialize, Deserialize)]
struct Offer {
    price: u64,
    item_id: ObjectID,
    user: SuiAddress,
    coin_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Registry {
    id: UID,
    sell_offers: Vec<Offer>,
    buy_offers: Vec<Offer>,
}

impl Package {
    pub fn new(package_id: &str, registry_id: &str) -> Result<Self> {
        Ok(Self {
            _package_id: ObjectID::from_str(package_id)?,
            registry_id: ObjectID::from_str(registry_id)?,
        })
    }

    pub async fn buy_cheapest_item(&self) -> Result<()> {
        let mut registry = self.get_registry().await?;
        let offer = Self::get_cheapest_sell_offer(&mut registry).await;
        if Option::is_some(&offer) {
            // TODO: fill sell offer
        }

        println!("buying...");

        Ok(())
    }

    async fn get_registry(&self) -> Result<Registry> {
        let client = sui_client().await?;
        let options = SuiObjectDataOptions::with_bcs(SuiObjectDataOptions::default());
        let object = client
            .read_api()
            .get_object_with_options(self.registry_id, options)
            .await?;

        if let SuiRawData::MoveObject(obj) = object.data.unwrap().bcs.unwrap() {
            Ok(obj.deserialize::<Registry>().unwrap())
        } else {
            bail!("Invalid registry object")
        }
    }

    async fn get_cheapest_sell_offer(registry: &mut Registry) -> Option<&Offer> {
        let offers = &mut registry.sell_offers;
        offers.sort_by(|a, b| a.price.cmp(&b.price));

        offers.get(0)
    }
}
