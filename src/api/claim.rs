use log::{debug, trace};
use reqwest::Result;
use serde::{Deserialize, Serialize};

use crate::api::BurrowApiResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct BurrowClaimArgs {
    contract_id: String,
    method_name: String,
    args: Option<serde_json::Value>,
}

pub async fn claim() -> Result<BurrowClaimArgs> {
    trace!("start");
    let resp = reqwest::Client::new()
        .post("https://api.burrow.finance/account_farm_claim_all")
        .send()
        .await?
        .json::<BurrowApiResponse<BurrowClaimArgs>>()
        .await?;
    debug!("{resp:#?}");

    trace!("finish");
    Ok(resp.data)
}
