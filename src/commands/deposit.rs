use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{BurrowApiResponse, Proposal, ProposalData, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct SupplyData {
    amount: f32,
    contract_id: String,
    method_name: String,
    args: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct BurrowSupplyBody {
    token_id: String,
    amount: String,
    is_collateral: bool,
}

pub async fn deposit(token_id: &String, amount: &String) -> Result<SupplyData> {
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
        .json::<BurrowApiResponse<SupplyData>>()
        .await?;
    // println!("{resp:#?}");

    let data = json!(resp.data);
    // println!("{data:#}");

    let proposal = Proposal {
        proposal: ProposalData {
            description: "Deposit to Burrow".into(),
            submission_time: "86400000000000".into(),
            kind: json!({
                "FunctionCall": data
            }),
        },
    };
    println!("{}", serde_json::to_string(&proposal)?);

    Ok(resp.data)
}
