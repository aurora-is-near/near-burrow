use std::error::Error;

use base64::prelude::*;
use clap::Parser;
use env_logger::Builder;
use near_burrow::{
    api,
    cli::{Cli, Commands},
    multisig::{to_proposal, ActionCall, FunctionCallData},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    let submission_time = "86400000000000".to_string();

    match &cli.command {
        Some(Commands::List {}) => {
            let resp = api::list().await?;
            println!("{}", resp);
        }

        Some(Commands::Register {
            token_id,
            account_id,
            amount,
        }) => {
            let action = api::register(token_id, account_id, amount).await?;
            let description = "Register account in Burrow".to_string();
            let mut base64_args = String::new();
            BASE64_STANDARD.encode_string(action.args.to_string(), &mut base64_args);
            let function_call_data = FunctionCallData {
                receiver_id: action.contract_id.to_string(),
                actions: vec![ActionCall {
                    method_name: action.method_name,
                    args: Some(base64_args),
                }],
            };
            let proposal_args = to_proposal(description, submission_time, function_call_data);
            println!("{}", proposal_args);
        }

        Some(Commands::Deposit { token_id, amount }) => {
            let action = api::deposit(token_id, amount).await?;
            let description = "Deposit to Burrow".to_string();
            let mut base64_args = String::new();
            BASE64_STANDARD.encode_string(action.args.to_string(), &mut base64_args);
            let function_call_data = FunctionCallData {
                receiver_id: action.contract_id.to_string(),
                actions: vec![ActionCall {
                    method_name: action.method_name,
                    args: Some(base64_args),
                }],
            };
            let proposal_args = to_proposal(description, submission_time, function_call_data);
            println!("{}", proposal_args);
        }

        Some(Commands::Withdraw { token_id, amount }) => {
            let action = api::withdraw(token_id, amount).await?;
            let description = "Withdraw from Burrow".to_string();
            let mut base64_args = String::new();
            BASE64_STANDARD.encode_string(action.args.to_string(), &mut base64_args);
            let function_call_data = FunctionCallData {
                receiver_id: action.contract_id.to_string(),
                actions: vec![ActionCall {
                    method_name: action.method_name,
                    args: Some(base64_args),
                }],
            };
            let proposal_args = to_proposal(description, submission_time, function_call_data);
            println!("{}", proposal_args);
        }

        Some(Commands::Claim {}) => {
            let action = api::claim().await?;
            let description = "Claim from Burrow".to_string();
            let function_call_data = FunctionCallData {
                receiver_id: action.contract_id.to_string(),
                actions: vec![ActionCall {
                    method_name: action.method_name,
                    args: None,
                }],
            };
            let proposal_args = to_proposal(description, submission_time, function_call_data);
            println!("{}", proposal_args);
        }
        None => {}
    };

    Ok(())
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
