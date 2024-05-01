use std::fmt::{self};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct ProposalArgs {
    pub proposal: Proposal,
}

impl fmt::Display for ProposalArgs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or_default())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Proposal {
    pub description: String,
    pub submission_time: String,
    pub kind: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct FunctionCallData {
    pub receiver_id: String,
    pub actions: Vec<ActionCall>,
}

#[derive(Serialize, Deserialize)]
pub struct ActionCall {
    pub method_name: String,
    pub args: Option<String>,
}

pub fn to_proposal(
    description: String,
    submission_time: String,
    data: FunctionCallData,
) -> ProposalArgs {
    ProposalArgs {
        proposal: Proposal {
            description,
            submission_time,
            kind: json!({
                "FunctionCall": data
            }),
        },
    }
}
