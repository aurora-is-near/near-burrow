use reqwest::Result;
use serde::{Deserialize, Serialize};

use crate::api::BurrowApiResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct BurrowSupplyArgs {
    amount: f32,
    contract_id: String,
    method_name: String,
    args: serde_json::Value,
}

#[derive(Serialize)]
struct BurrowSupplyBody {
    token_id: String,
    amount: String,
    is_collateral: bool,
}

pub async fn deposit(token_id: &String, amount: &String) -> Result<BurrowSupplyArgs> {
    let body = BurrowSupplyBody {
        token_id: token_id.into(),
        amount: amount.into(),
        is_collateral: true,
    };

    let resp = reqwest::Client::new()
        .post("https://api.burrow.finance/supply")
        .json(&body)
        .send()
        .await?
        .json::<BurrowApiResponse<BurrowSupplyArgs>>()
        .await?;

    Ok(resp.data)
}
