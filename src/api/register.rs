use log::{debug, trace};
use reqwest::Result;
use serde::{Deserialize, Serialize};

use crate::api::BurrowApiResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct BurrowRegisterArgs {
    contract_id: String,
    method_name: String,
    args: Option<serde_json::Value>,
}

#[derive(Serialize)]
struct BurrowRegisterBody {
    token_id: String,
    account_id: String,
    amount: String,
}

pub async fn register(
    token_id: &String,
    account_id: &String,
    amount: &String,
) -> Result<BurrowRegisterArgs> {
    trace!("start");
    let body = BurrowRegisterBody {
        token_id: token_id.into(),
        account_id: account_id.into(),
        amount: amount.into(),
    };

    let resp = reqwest::Client::new()
        .post("https://api.burrow.finance/storage_deposit")
        .json(&body)
        .send()
        .await?
        .json::<BurrowApiResponse<BurrowRegisterArgs>>()
        .await?;
    debug!("response {resp:#?}");

    trace!("finish");
    Ok(resp.data)
}
