use std::fmt::{self};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct ProposalArgs {
    pub proposal: Proposal,
}

impl fmt::Display for ProposalArgs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).unwrap_or(String::default())
        )
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
            description: description.into(),
            submission_time: submission_time.into(),
            kind: json!({
                "FunctionCall": data
            }),
        },
    }
}
