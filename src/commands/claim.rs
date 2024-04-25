use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{BurrowApiResponse, Proposal, ProposalData, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimData {
    contract_id: String,
    method_name: String,
    args: Option<serde_json::Value>,
}

pub async fn claim() -> Result<ClaimData> {
    let resp = reqwest::Client::new()
        .post("https://api.burrow.finance/account_farm_claim_all")
        .send()
        .await?
        .json::<BurrowApiResponse<ClaimData>>()
        .await?;
    // println!("{resp:#?}");

    let data = json!(resp.data);
    // println!("{data:#}");

    let proposal = Proposal {
        proposal: ProposalData {
            description: "Claim from Burrow".into(),
            submission_time: "86400000000000".into(),
            kind: json!({
                "FunctionCall": data
            }),
        },
    };
    println!("{}", serde_json::to_string(&proposal)?);

    Ok(resp.data)
}
