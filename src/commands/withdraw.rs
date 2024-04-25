use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{BurrowApiResponse, Proposal, ProposalData, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawData {
    amount: f32,
    contract_id: String,
    method_name: String,
    args: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct BurrowWithdrawBody {
    token_id: String,
    amount: String,
}

pub async fn withdraw(token_id: &String, amount: &String) -> Result<WithdrawData> {
    let body = BurrowWithdrawBody {
        token_id: token_id.into(),
        amount: amount.into(),
    };

    let resp = reqwest::Client::new()
        .post("https://api.burrow.finance/withdraw")
        .json(&body)
        .send()
        .await?
        .json::<BurrowApiResponse<WithdrawData>>()
        .await?;
    // println!("{resp:#?}");

    let data = json!(resp.data);
    // println!("{data:#}");

    let proposal = Proposal {
        proposal: ProposalData {
            description: "Withdraw from Burrow".into(),
            submission_time: "86400000000000".into(),
            kind: json!({
                "FunctionCall": data
            }),
        },
    };
    println!("{}", serde_json::to_string(&proposal)?);

    Ok(resp.data)
}
