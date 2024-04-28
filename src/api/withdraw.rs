use log::{debug, trace};
use reqwest::Result;
use serde::{Deserialize, Serialize};

use crate::api::BurrowApiResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct BurrowWithdrawArgs {
    amount: f32,
    contract_id: String,
    method_name: String,
    args: serde_json::Value,
}

#[derive(Serialize)]
struct BurrowWithdrawBody {
    token_id: String,
    amount: String,
}

pub async fn withdraw(token_id: &String, amount: &String) -> Result<BurrowWithdrawArgs> {
    trace!("start");
    let body = BurrowWithdrawBody {
        token_id: token_id.into(),
        amount: amount.into(),
    };

    let resp = reqwest::Client::new()
        .post("https://api.burrow.finance/withdraw")
        .json(&body)
        .send()
        .await?
        .json::<BurrowApiResponse<BurrowWithdrawArgs>>()
        .await?;
    debug!("response {resp:#?}");

    trace!("finish");
    Ok(resp.data)
}
