use std::fmt;

use log::{debug, trace};
use reqwest::Result;
use serde::{Deserialize, Serialize};

use crate::api::BurrowApiResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetMarkets(Vec<AssetMarketData>);

impl fmt::Display for AssetMarkets {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or(String::default())
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetMarketData {
    token_id: String,
    borrow_apr: String,
    supply_apr: String,
    supplied: MarketCollateralData,
    borrowed: MarketCollateralData,
    config: MarketConfigData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct MarketCollateralData {
    balance: String,
    shares: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct MarketConfigData {
    can_borrow: bool,
    can_deposit: bool,
    can_use_as_collateral: bool,
    can_withdraw: bool,
}

pub async fn list() -> Result<AssetMarkets> {
    trace!("start");
    let resp = reqwest::get("https://api.burrow.finance/get_assets_paged_detailed")
        .await?
        .json::<BurrowApiResponse<AssetMarkets>>()
        .await?;
    debug!("response {resp:#?}");

    trace!("finish");
    Ok(resp.data)
}
