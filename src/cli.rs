use clap::{command, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "near-burrow")]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
pub struct Cli {
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// A command to print all asset markets
    List {},
    /// A command to compute a register transaction for an account in Burrow
    Register {
        /// The account_id of the token to be registered
        token_id: String,
        /// The account_id of the signer that will be registered
        account_id: String,
        /// Storage fee in yoctoNEAR
        amount: String,
    },
    /// A command to compute a deposit transaction
    Deposit {
        /// The account_id of the token to be deposited
        token_id: String,
        /// Amount of tokens to deposit
        amount: String,
    },
    /// A command to compute a withdraw transaction
    Withdraw {
        /// The account_id of the token to be withdrawn
        token_id: String,
        /// Amount of tokens to withdraw
        amount: String,
    },
    /// A command to compute a claim transaction
    Claim {},
}
