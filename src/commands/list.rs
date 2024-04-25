use reqwest::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::BurrowApiResponse;

type AssetMarkets = Option<Vec<AssetMarketData>>;

#[derive(Debug, Serialize, Deserialize)]
struct AssetMarketData {
    token_id: String,
    borrow_apr: String,
    supply_apr: String,
    supplied: MarketCollateralData,
    borrowed: MarketCollateralData,
    config: MarketConfigData,
}

#[derive(Debug, Serialize, Deserialize)]
struct MarketCollateralData {
    balance: String,
    shares: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MarketConfigData {
    can_borrow: bool,
    can_deposit: bool,
    can_use_as_collateral: bool,
    can_withdraw: bool,
}

pub async fn list() -> Result<()> {
    let resp = reqwest::get("https://api.burrow.finance/get_assets_paged_detailed")
        .await?
        .json::<BurrowApiResponse<AssetMarkets>>()
        .await?;
    // println!("{resp:#?}");

    let json = json!(resp.data);
    println!("{json:#}");

    Ok(())
}
