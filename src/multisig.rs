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

pub fn to_proposal<T: Serialize>(
    description: String,
    submission_time: String,
    data: T,
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
