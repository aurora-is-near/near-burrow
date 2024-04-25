use serde::{Deserialize, Serialize};

pub mod commands;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BurrowApiResponse<T> {
    code: String,
    msg: String,
    data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proposal {
    proposal: ProposalData,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProposalData {
    description: String,
    submission_time: String,
    kind: serde_json::Value,
}
