use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{BurrowApiResponse, Proposal, ProposalData, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterData {
    contract_id: String,
    method_name: String,
    args: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BurrowRegisterBody {
    token_id: String,
    account_id: String,
    amount: String,
}

pub async fn register(
    token_id: &String,
    account_id: &String,
    amount: &String,
) -> Result<RegisterData> {
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
        .json::<BurrowApiResponse<RegisterData>>()
        .await?;
    // println!("{resp:#?}");

    let data = json!(resp.data);
    // println!("{data:#}");

    let proposal = Proposal {
        proposal: ProposalData {
            description: "Register account in Burrow".into(),
            submission_time: "86400000000000".into(),
            kind: json!({
                "FunctionCall": data
            }),
        },
    };
    println!("{}", serde_json::to_string(&proposal)?);

    Ok(resp.data)
}