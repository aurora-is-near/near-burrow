use clap::{Parser, Subcommand};
use near_burrow::{commands, Result};

#[derive(Parser)]
#[command(name = "near-burrow")]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    // match cli.debug {
    //     0 => println!("Debug mode is off"),
    //     1 => println!("Debug mode is kind of on"),
    //     2 => println!("Debug mode is on"),
    //     _ => println!("Don't be crazy"),
    // }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::List {}) => {
            commands::list().await?;
        }
        Some(Commands::Register {
            token_id,
            account_id,
            amount,
        }) => {
            commands::register(token_id, account_id, amount).await?;
        }
        Some(Commands::Deposit { token_id, amount }) => {
            commands::deposit(token_id, amount).await?;
        }
        Some(Commands::Withdraw { token_id, amount }) => {
            commands::withdraw(token_id, amount).await?;
        }
        Some(Commands::Claim {}) => {
            commands::claim().await?;
        }
        None => {}
    }

    // Continued program logic goes here...
    Ok(())
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
